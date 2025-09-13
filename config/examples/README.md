# ETL 配置範例

這個目錄包含了各種 ETL 配置範例，展示如何使用通用配置系統來處理不同的資料源和輸出格式。

## 配置文件結構

每個配置文件都包含以下主要部分：

### 1. 基本資訊
- `name`: 配置的名稱
- `description`: 配置的描述（可選）

### 2. 資料源 (`data_source`)
支援的資料源類型：
- **API**: REST API 端點
- **本地文件**: CSV、JSON、Excel、Parquet 等
- **資料庫**: PostgreSQL、MySQL、SQLite、SurrealDB
- **雲存儲**: S3 等

### 3. 資料轉換 (`transformations`)
支援的轉換類型：
- **Map**: 值映射轉換
- **Calculate**: 數學計算和表達式
- **Format**: 字符串格式化
- **Convert**: 資料類型轉換
- **Filter**: 資料過濾
- **Aggregate**: 聚合運算
- **Join**: 資料連接
- **Custom**: 自定義函數

### 4. 輸出配置 (`output`)
支援的輸出格式：
- **CSV/TSV**: 分隔符文件
- **JSON**: JSON 格式
- **Excel**: Excel 電子表格
- **Parquet**: 高效列式存儲
- **Database**: 直接寫入資料庫

### 5. 全局設定 (`settings`)
- 並行處理配置
- 記憶體限制
- 日誌等級
- 超時設定
- 環境變數

## 範例文件說明

### 1. `api_to_csv_example.json`
展示從 API 提取資料並轉換為 CSV 格式的完整流程。
- 支援 API 認證
- 重試機制
- 資料映射和計算
- 壓縮輸出

### 2. `csv_transform_example.json` 
處理本地 CSV 文件並輸出到資料庫。
- CSV 解析配置
- 複雜資料轉換
- 聚合運算
- 資料庫寫入

### 3. `database_to_json_example.json`
從資料庫查詢資料並導出為 JSON。
- 資料庫連接
- 自定義 SQL 查詢
- 資料脫敏
- S3 輸出

### 4. `complex_zip_processing.json`
處理壓縮文件中的多個文件。
- ZIP 文件解壓
- 多文件合併
- 資料連接
- 分組聚合

## 使用方式

```bash
# 使用配置文件運行 ETL
cargo run -- --config config/examples/api_to_csv_example.json

# 驗證配置文件
cargo run -- --validate config/examples/csv_transform_example.json

# 查看配置模板
cargo run -- --template > my_config.json
```

## 環境變數

配置中可以使用環境變數，格式為 `${VARIABLE_NAME}`：

```json
{
  "auth": {
    "credentials": {
      "token": "${API_TOKEN}"
    }
  }
}
```

## 自定義函數

對於特殊需求，可以使用自定義轉換函數：

```json
{
  "transformation": {
    "type": "custom",
    "function": "your_custom_function",
    "parameters": {
      "param1": "value1"
    }
  }
}
```

## 最佳實踐

1. **模組化配置**: 將複雜的轉換分解為多個簡單步驟
2. **錯誤處理**: 使用條件過濾避免處理無效資料
3. **效能優化**: 合理設置批次大小和並行處理數
4. **安全性**: 使用環境變數存儲敏感資訊
5. **可讀性**: 添加有意義的名稱和描述

## 擴展性

這個配置系統設計為高度可擴展：
- 新增資料源類型
- 新增轉換函數
- 新增輸出格式
- 自定義驗證規則