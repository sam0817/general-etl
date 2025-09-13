use crate::models::data_types::DataRecord;
use crate::utils::error::Result;
use csv::WriterBuilder;
use std::fs::File;
use std::path::Path;

pub struct CsvWriter {
    delimiter: u8,
}

impl CsvWriter {
    pub fn new(delimiter: u8) -> Self {
        Self { delimiter }
    }

    pub fn write_records<P: AsRef<Path>>(
        &self,
        path: P,
        records: &[DataRecord],
        headers: &[String],
    ) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = WriterBuilder::new()
            .delimiter(self.delimiter)
            .from_writer(file);

        // 寫入標題
        writer.write_record(headers)?;

        // 寫入資料
        for record in records {
            let row: Vec<String> = headers
                .iter()
                .map(|header| {
                    record.fields
                        .get(header)
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                })
                .collect();

            writer.write_record(&row)?;
        }

        writer.flush()?;
        Ok(())
    }
}
