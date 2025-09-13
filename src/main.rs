use general_etl::models::data_types::{MappingRule, TransformationType};
use general_etl::pipeline::{DataSource, EtlPipeline, OutputConfig, OutputFormat, PipelineConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日誌
    tracing_subscriber::fmt::init();

    // 建立 ETL 管線
    let pipeline = EtlPipeline::new("https://api.example.com".to_string());

    // 設定管線配置
    let config = PipelineConfig {
        source: DataSource::Api("data/endpoint".to_string()),
        mapping_file: "config/mappings/product_mapping.json".to_string(),
        rules: vec![
            MappingRule {
                source_field: "product_name".to_string(),
                target_field: "name".to_string(),
                transformation: Some(TransformationType::Uppercase),
            },
            MappingRule {
                source_field: "category_id".to_string(),
                target_field: "category".to_string(),
                transformation: Some(TransformationType::Lookup("category".to_string())),
            },
        ],
        output: OutputConfig {
            format: OutputFormat::Csv,
            path: "output/processed_data.zip".to_string(),
            compress: true,
        },
    };

    // 執行管線
    pipeline.run(config).await?;

    Ok(())
}
