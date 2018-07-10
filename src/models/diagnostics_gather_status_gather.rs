

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsGatherStatusGather {
  #[serde(rename = "path")]
  path: Option<String>,
  /// 
  #[serde(rename = "status")]
  status: Option<::models::DiagnosticsGatherStatusGatherStatus>
}

