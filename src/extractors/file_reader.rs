use crate::utils::error::{EtlError, Result};
use crate::config::settings::FileFormat;
use crate::models::data_types::DataRecord;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::BufReader;
use zip::ZipArchive;
use flate2::read::GzDecoder;

pub struct FileReader {
    encoding: String,
}

impl FileReader {
    pub fn new() -> Self {
        Self {
            encoding: "utf-8".to_string(),
        }
    }

    pub fn with_encoding(encoding: String) -> Self {
        Self { encoding }
    }

    pub async fn read_file(
        &self,
        path: &str,
        format: FileFormat,
    ) -> Result<Vec<DataRecord>> {
        match format {
            FileFormat::Json => self.read_json(path).await,
            FileFormat::Csv { delimiter, has_headers } => {
                self.read_csv(path, delimiter.unwrap_or(','), has_headers.unwrap_or(true)).await
            }
            FileFormat::Tsv => {
                self.read_csv(path, '\t', true).await
            }
            FileFormat::Excel => {
                Err(EtlError::ConfigError("Excel format not yet implemented".to_string()))
            }
            FileFormat::Parquet => {
                Err(EtlError::ConfigError("Parquet format not yet implemented".to_string()))
            }
            FileFormat::Zip { extract_path: _, target_files } => {
                self.read_zip(path, target_files).await
            }
        }
    }

    async fn read_json(&self, path: &str) -> Result<Vec<DataRecord>> {
        let content = read_to_string(path)?;
        let json_value: serde_json::Value = serde_json::from_str(&content)?;
        
        match json_value {
            serde_json::Value::Array(array) => {
                let mut records = Vec::new();
                for (index, item) in array.into_iter().enumerate() {
                    records.push(self.json_to_record(item, index)?);
                }
                Ok(records)
            }
            serde_json::Value::Object(_) => {
                Ok(vec![self.json_to_record(json_value, 0)?])
            }
            _ => Err(EtlError::ParseError("JSON must be object or array".to_string())),
        }
    }

    async fn read_csv(
        &self,
        path: &str,
        delimiter: char,
        has_headers: bool,
    ) -> Result<Vec<DataRecord>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(delimiter as u8)
            .has_headers(has_headers)
            .from_reader(reader);

        let headers = if has_headers {
            csv_reader.headers()?.iter().map(|h| h.to_string()).collect::<Vec<_>>()
        } else {
            (0..csv_reader.headers()?.len())
                .map(|i| format!("column_{}", i))
                .collect()
        };

        let mut records = Vec::new();
        for result in csv_reader.records() {
            let record = result?;
            let mut fields = HashMap::new();
            
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    let value = self.parse_csv_field(field);
                    fields.insert(header.clone(), value);
                }
            }

            records.push(DataRecord { fields });
        }

        Ok(records)
    }

    async fn read_zip(&self, path: &str, target_files: Vec<String>) -> Result<Vec<DataRecord>> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut all_records = Vec::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            let should_process = if target_files.is_empty() {
                true
            } else {
                target_files.iter().any(|pattern| {
                    self.matches_pattern(&file_name, pattern)
                })
            };

            if should_process {
                let mut contents = Vec::new();
                std::io::copy(&mut file, &mut contents)?;
                let contents = String::from_utf8(contents).map_err(|e| {
                    EtlError::ParseError(format!("Invalid UTF-8 in file {}: {}", file_name, e))
                })?;
                
                if file_name.ends_with(".csv") {
                    let temp_path = format!("temp_{}", file_name);
                    std::fs::write(&temp_path, &contents)?;
                    
                    let records = self.read_csv(&temp_path, ',', true).await?;
                    all_records.extend(records);
                    
                    let _ = std::fs::remove_file(&temp_path);
                    
                } else if file_name.ends_with(".json") {
                    let json_value: serde_json::Value = serde_json::from_str(&contents)?;
                    match json_value {
                        serde_json::Value::Array(array) => {
                            for (index, item) in array.into_iter().enumerate() {
                                all_records.push(self.json_to_record(item, index)?);
                            }
                        }
                        serde_json::Value::Object(_) => {
                            all_records.push(self.json_to_record(json_value, 0)?);
                        }
                        _ => continue,
                    }
                }
            }
        }

        Ok(all_records)
    }

    fn json_to_record(&self, json: serde_json::Value, index: usize) -> Result<DataRecord> {
        let mut fields = HashMap::new();
        
        match json {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    fields.insert(key, value);
                }
            }
            _ => {
                fields.insert("value".to_string(), json);
                fields.insert("_index".to_string(), serde_json::Value::Number(index.into()));
            }
        }

        Ok(DataRecord { fields })
    }

    fn parse_csv_field(&self, field: &str) -> serde_json::Value {
        if let Ok(int_val) = field.parse::<i64>() {
            return serde_json::Value::Number(int_val.into());
        }
        
        if let Ok(float_val) = field.parse::<f64>() {
            if let Some(num) = serde_json::Number::from_f64(float_val) {
                return serde_json::Value::Number(num);
            }
        }

        match field.to_lowercase().as_str() {
            "true" | "yes" | "1" => serde_json::Value::Bool(true),
            "false" | "no" | "0" => serde_json::Value::Bool(false),
            _ => serde_json::Value::String(field.to_string()),
        }
    }

    fn matches_pattern(&self, filename: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                filename.starts_with(parts[0]) && filename.ends_with(parts[1])
            } else {
                false
            }
        } else {
            filename == pattern
        }
    }
}