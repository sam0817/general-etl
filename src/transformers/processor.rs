use crate::models::data_types::{DataRecord, MappingRule, TransformationType};
use crate::utils::error::Result;
use dashmap::DashMap;
use rayon::prelude::*;

pub struct DataProcessor {
    mapping_cache: DashMap<String, String>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            mapping_cache: DashMap::new(),
        }
    }

    pub fn load_mapping(&self, mapping_data: Vec<(String, String)>) {
        for (key, value) in mapping_data {
            self.mapping_cache.insert(key, value);
        }
    }

    pub fn process_records(
        &self,
        records: Vec<DataRecord>,
        rules: &[MappingRule],
    ) -> Result<Vec<DataRecord>> {
        let processed: Vec<DataRecord> = records
            .par_iter()
            .map(|record| self.apply_rules(record, rules))
            .collect::<Result<Vec<_>>>()?;

        Ok(processed)
    }

    fn apply_rules(&self, record: &DataRecord, rules: &[MappingRule]) -> Result<DataRecord> {
        let mut new_record = DataRecord {
            fields: std::collections::HashMap::new(),
        };

        for rule in rules {
            if let Some(value) = record.fields.get(&rule.source_field) {
                let transformed_value = match &rule.transformation {
                    Some(TransformationType::Uppercase) => {
                        serde_json::Value::String(
                            value.as_str()
                                .unwrap_or("")
                                .to_uppercase()
                        )
                    }
                    Some(TransformationType::Lowercase) => {
                        serde_json::Value::String(
                            value.as_str()
                                .unwrap_or("")
                                .to_lowercase()
                        )
                    }
                    Some(TransformationType::Calculate(expr)) => {
                        self.calculate_field(value, expr)?
                    }
                    Some(TransformationType::Lookup(mapping_name)) => {
                        self.lookup_value(value, mapping_name)?
                    }
                    None => value.clone(),
                };

                new_record.fields.insert(rule.target_field.clone(), transformed_value);
            }
        }

        Ok(new_record)
    }

    fn calculate_field(&self, value: &serde_json::Value, expr: &str) -> Result<serde_json::Value> {
        // 實作簡單計算邏輯
        // 可以整合 evalexpr 套件來處理複雜運算式
        Ok(value.clone())
    }

    fn lookup_value(&self, value: &serde_json::Value, mapping_name: &str) -> Result<serde_json::Value> {
        let key = value.as_str().unwrap_or("");
        let lookup_key = format!("{}:{}", mapping_name, key);

        if let Some(mapped_value) = self.mapping_cache.get(&lookup_key) {
            Ok(serde_json::Value::String(mapped_value.clone()))
        } else {
            Ok(value.clone())
        }
    }
}
