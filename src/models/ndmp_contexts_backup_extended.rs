
/// NdmpContextsBackupExtended : Get NDMP Context List

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBackupExtended {
  #[serde(rename = "contexts")]
  contexts: Option<Vec<::models::NdmpContextsBackupContext>>,
  /// Resume string returned by previous query.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// The number of ndmp contexts.
  #[serde(rename = "total")]
  total: Option<i32>
}

