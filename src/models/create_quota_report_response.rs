

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuotaReportResponse {
  /// ID of created item that can be used to refer to item in the collection-item resource path.
  #[serde(rename = "id")]
  id: i32
}

