use general_etl::models::data_types::{MappingRule, TransformationType};
use general_etl::pipeline::{DataSource, EtlPipeline, OutputConfig, OutputFormat, PipelineConfig};

#[tokio::main]
async fn main_2() -> anyhow::Result<()> {
    // 初始化日誌
    tracing_subscriber::fmt::init();

    // 建立 ETL 管線
    let pipeline = EtlPipeline::new("https://sampleapis.com/switch/games".to_string());

    // 設定管線配置
    let config = PipelineConfig {
        source: DataSource::Api("switch/games".to_string()),
        mapping_file: "config/mappings/product_mapping.json".to_string(),
        rules: vec![
            MappingRule {
                source_field: "name".to_string(),
                target_field: "game_name".to_string(),
                transformation: Some(TransformationType::Uppercase),
            },
            MappingRule {
                source_field: "id".to_string(),
                target_field: "source_id".to_string(),
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
    // pipeline.run(config).await?;
    // main_surreal_test()?;
    Ok(())
}

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::Surreal;

// For an in memory database
use surrealdb::engine::local::Mem;

// For a RocksDB file
// use surrealdb::engine::local::RocksDb;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: RecordId,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection in memory
    let db = Surreal::new::<Mem>(()).await?;

    // Create database connection using RocksDB
    // let db = Surreal::new::<RocksDb>("path/to/database-folder").await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Option<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    let updated: Option<Record> = db
        .update(("person", "jaime"))
        .merge(Responsibility { marketing: true })
        .await?;
    dbg!(updated);

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    Ok(())
}
