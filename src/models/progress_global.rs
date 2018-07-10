

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressGlobal {
  /// Lists the current global audit log times.
  #[serde(rename = "progress")]
  progress: Option<::models::ProgressGlobalProgress>
}

