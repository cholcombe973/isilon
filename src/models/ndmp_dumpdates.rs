
/// NdmpDumpdates : Get list of dumpdates entries.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpDumpdates {
  #[serde(rename = "dumpdates")]
  dumpdates: Option<Vec<::models::NdmpDumpdate>>,
  /// Resume string returned by previous query.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// The number of dumpdates entries.
  #[serde(rename = "total")]
  total: Option<i32>
}

