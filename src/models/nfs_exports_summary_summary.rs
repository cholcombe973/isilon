

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsExportsSummarySummary {
  /// The count of objects in the collection
  #[serde(rename = "count")]
  count: i32
}

