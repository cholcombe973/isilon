

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportAboutReport {
  /// Whether report was manually requested (live) or scheduled.
  #[serde(rename = "generated")]
  generated: String,
  /// The system ID given to the report.
  #[serde(rename = "id")]
  id: String,
  /// Unix epoch time the report was taken.
  #[serde(rename = "time")]
  time: i32,
  /// Whether this is a summary or detail report.
  #[serde(rename = "type")]
  _type: String
}

