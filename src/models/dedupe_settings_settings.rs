

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeSettingsSettings {
  /// The paths that will be assessed.
  #[serde(rename = "assess_paths")]
  assess_paths: Option<Vec<String>>,
  /// The schedule for the dedupe job.
  #[serde(rename = "dedupe_schedule")]
  dedupe_schedule: Option<String>,
  /// The paths that will be deduped.
  #[serde(rename = "paths")]
  paths: Option<Vec<String>>
}

