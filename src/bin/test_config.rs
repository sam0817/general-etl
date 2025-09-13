use general_etl::config::settings::EtlConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日誌
    tracing_subscriber::fmt::init();

    // 測試配置載入
    println!("Testing configuration loading...");
    
    let config = EtlConfig::from_file("test_config.json")?;
    println!("✅ Configuration loaded successfully!");
    println!("Config name: {}", config.name);
    println!("Description: {}", config.description.as_ref().unwrap_or(&"None".to_string()));
    
    // 驗證配置
    match config.validate() {
        Ok(()) => println!("✅ Configuration is valid!"),
        Err(e) => {
            println!("❌ Configuration validation failed: {}", e);
            return Err(e.into());
        }
    }

    // 測試配置序列化
    println!("\nTesting configuration serialization...");
    let json_output = serde_json::to_string_pretty(&config)?;
    println!("✅ Configuration serialized successfully!");
    println!("Serialized length: {} bytes", json_output.len());

    // 測試文件讀取器
    println!("\nTesting file reader...");
    use general_etl::extractors::file_reader::FileReader;
    use general_etl::config::settings::FileFormat;
    
    let file_reader = FileReader::new();
    match file_reader.read_file("test_data.json", FileFormat::Json).await {
        Ok(records) => {
            println!("✅ File reading successful!");
            println!("Records loaded: {}", records.len());
            
            for (i, record) in records.iter().enumerate() {
                println!("  Record {}: {} fields", i + 1, record.fields.len());
            }
        }
        Err(e) => {
            println!("❌ File reading failed: {}", e);
            return Err(e.into());
        }
    }

    println!("\n🎉 All tests passed!");
    Ok(())
}