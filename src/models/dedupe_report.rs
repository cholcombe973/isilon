

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeReport {
  /// The phase of the job this report was generated for.
  #[serde(rename = "phase")]
  phase: Option<i32>,
  /// The report results.
  #[serde(rename = "results")]
  results: Option<String>,
  /// The time this report was generated in Unix epoch seconds.
  #[serde(rename = "time")]
  time: Option<i32>
}

