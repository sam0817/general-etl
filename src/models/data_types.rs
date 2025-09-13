use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ProcessedData {
    pub records: Vec<DataRecord>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub source: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub record_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MappingRule {
    pub source_field: String,
    pub target_field: String,
    pub transformation: Option<TransformationType>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformationType {
    Uppercase,
    Lowercase,
    Calculate(String),
    Lookup(String),
}
