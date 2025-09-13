use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtlConfig {
    pub name: String,
    pub description: Option<String>,
    pub data_source: DataSourceConfig,
    pub transformations: Vec<TransformationConfig>,
    pub output: OutputConfig,
    pub settings: Option<GlobalSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataSourceConfig {
    Api {
        url: String,
        method: Option<String>,
        headers: Option<HashMap<String, String>>,
        auth: Option<AuthConfig>,
        retry: Option<RetryConfig>,
    },
    LocalFile {
        path: String,
        format: FileFormat,
        encoding: Option<String>,
    },
    Database {
        connection_string: String,
        query: String,
        driver: DatabaseDriver,
    },
    S3 {
        bucket: String,
        key: String,
        region: String,
        credentials: Option<AwsCredentials>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileFormat {
    Json,
    Csv {
        delimiter: Option<char>,
        has_headers: Option<bool>,
    },
    Tsv,
    Excel,
    Parquet,
    Zip {
        extract_path: Option<String>,
        target_files: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseDriver {
    Postgres,
    Mysql,
    Sqlite,
    Surreal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub auth_type: AuthType,
    pub credentials: AuthCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthType {
    BasicAuth,
    BearerToken,
    ApiKey,
    OAuth2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub api_key: Option<String>,
    pub header_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationConfig {
    pub name: String,
    pub source_field: String,
    pub target_field: Option<String>,
    pub transformation: TransformationType,
    pub condition: Option<ConditionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransformationType {
    Map {
        mapping: HashMap<String, String>,
    },
    Calculate {
        expression: String,
    },
    Format {
        template: String,
    },
    Convert {
        to_type: DataType,
    },
    Filter {
        condition: String,
    },
    Aggregate {
        operation: AggregateOperation,
        group_by: Option<Vec<String>>,
    },
    Join {
        join_source: String,
        join_key: String,
        join_type: JoinType,
    },
    Custom {
        function: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregateOperation {
    Count,
    Sum,
    Average,
    Min,
    Max,
    GroupConcat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub field: String,
    pub operator: ComparisonOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Contains,
    StartsWith,
    EndsWith,
    Regex,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub destination: OutputDestination,
    pub options: Option<OutputOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Csv {
        delimiter: Option<char>,
        quote_char: Option<char>,
        headers: Option<bool>,
    },
    Json {
        pretty_print: Option<bool>,
    },
    Excel {
        sheet_name: Option<String>,
    },
    Parquet,
    Database {
        table_name: String,
        mode: WriteMode,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputDestination {
    LocalFile {
        path: String,
        compress: Option<CompressionType>,
    },
    S3 {
        bucket: String,
        key: String,
        region: String,
        credentials: Option<AwsCredentials>,
    },
    Database {
        connection_string: String,
        driver: DatabaseDriver,
    },
    Api {
        url: String,
        method: Option<String>,
        headers: Option<HashMap<String, String>>,
        auth: Option<AuthConfig>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompressionType {
    Gzip,
    Zip,
    Bzip2,
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteMode {
    Overwrite,
    Append,
    Upsert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOptions {
    pub batch_size: Option<usize>,
    pub max_file_size: Option<u64>,
    pub split_by_field: Option<String>,
    pub filename_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub parallel_workers: Option<usize>,
    pub memory_limit_mb: Option<u64>,
    pub temp_directory: Option<String>,
    pub log_level: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub variables: Option<HashMap<String, String>>,
}

impl EtlConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: EtlConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Config name cannot be empty".to_string());
        }
        
        if self.transformations.is_empty() {
            return Err("At least one transformation must be specified".to_string());
        }
        
        Ok(())
    }
}