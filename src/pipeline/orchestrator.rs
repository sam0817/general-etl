use crate::extractors::api_client::ApiClient;
use crate::transformers::processor::DataProcessor;
use crate::loaders::{csv_writer::CsvWriter, archiver::Archiver};
use crate::models::data_types::{DataRecord, ProcessedData, MappingRule};
use crate::utils::error::Result;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::{info, error};

pub struct EtlPipeline {
    api_client: ApiClient,
    processor: DataProcessor,
    csv_writer: CsvWriter,
}

impl EtlPipeline {
    pub fn new(base_url: String) -> Self {
        Self {
            api_client: ApiClient::new(),
            processor: DataProcessor::new(),
            csv_writer: CsvWriter::new(b','),
        }
    }

    pub async fn run(&self, config: PipelineConfig) -> Result<()> {
        let pb = ProgressBar::new(4);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
        );

        // 步驟 1: 擷取資料
        pb.set_message("Extracting data...");
        let raw_data = self.extract_data(&config.source).await?;
        pb.inc(1);

        // 步驟 2: 載入 mapping
        pb.set_message("Loading mappings...");
        self.load_mappings(&config.mapping_file).await?;
        pb.inc(1);

        // 步驟 3: 轉換資料
        pb.set_message("Transforming data...");
        let processed = self.transform_data(raw_data, &config.rules)?;
        pb.inc(1);

        // 步驟 4: 輸出結果
        pb.set_message("Writing output...");
        self.write_output(processed, &config.output).await?;
        pb.inc(1);

        pb.finish_with_message("ETL pipeline completed successfully!");
        Ok(())
    }

    async fn extract_data(&self, source: &DataSource) -> Result<Vec<DataRecord>> {
        match source {
            DataSource::Api(endpoint) => {
                let json = self.api_client.fetch_json(endpoint, None, None, None, None).await?;
                self.parse_json_to_records(json)
            }
            DataSource::CsvApi(endpoint) => {
                let csv_data = self.api_client.fetch_text(endpoint, None, None, None, None).await?;
                self.parse_csv_to_records(&csv_data)
            }
            DataSource::ZipApi(endpoint) => {
                let zip_data = self.api_client.fetch_bytes(endpoint, None, None, None, None).await?;
                self.extract_and_parse_zip(zip_data)
            }
        }
    }

    fn parse_json_to_records(&self, json: serde_json::Value) -> Result<Vec<DataRecord>> {
        // 實作 JSON 解析邏輯
        Ok(vec![])
    }

    fn parse_csv_to_records(&self, csv_data: &str) -> Result<Vec<DataRecord>> {
        // 實作 CSV 解析邏輯
        Ok(vec![])
    }

    fn extract_and_parse_zip(&self, zip_data: Vec<u8>) -> Result<Vec<DataRecord>> {
        // 實作 ZIP 解壓縮和解析邏輯
        Ok(vec![])
    }

    async fn load_mappings(&self, mapping_file: &str) -> Result<()> {
        // 載入 mapping 檔案
        Ok(())
    }

    fn transform_data(
        &self,
        records: Vec<DataRecord>,
        rules: &[MappingRule],
    ) -> Result<ProcessedData> {
        let processed_records = self.processor.process_records(records, rules)?;

        Ok(ProcessedData {
            records: processed_records.clone(),
            metadata: crate::models::data_types::Metadata {
                source: "API".to_string(),
                timestamp: chrono::Utc::now(),
                record_count: processed_records.len(),
            },
        })
    }

    async fn write_output(&self, data: ProcessedData, output: &OutputConfig) -> Result<()> {
        // 實作輸出邏輯
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub source: DataSource,
    pub mapping_file: String,
    pub rules: Vec<MappingRule>,
    pub output: OutputConfig,
}

#[derive(Debug, Clone)]
pub enum DataSource {
    Api(String),
    CsvApi(String),
    ZipApi(String),
}

#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub path: String,
    pub compress: bool,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Csv,
    Tsv,
}
