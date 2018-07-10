

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudJobFilesName {
  /// The full path name of a file to be acted on by this job
  #[serde(rename = "name")]
  name: Option<String>,
  /// The current state of this file
  #[serde(rename = "state")]
  state: Option<String>
}

