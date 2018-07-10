

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobEvent {
  /// Event flags.
  #[serde(rename = "flags")]
  flags: String,
  /// A string representation of the type of the data value.
  #[serde(rename = "fmt_type")]
  fmt_type: String,
  /// Job event ID.
  #[serde(rename = "id")]
  id: i32,
  /// Job ID.
  #[serde(rename = "job_id")]
  job_id: i32,
  /// Job Type.
  #[serde(rename = "job_type")]
  job_type: String,
  /// Event key name.
  #[serde(rename = "key")]
  key: String,
  /// Job phase number at time of event.
  #[serde(rename = "phase")]
  phase: i32,
  /// An integer representation of the type of the data value.
  #[serde(rename = "raw_type")]
  raw_type: i32,
  /// Time of event in Unix epoch seconds.
  #[serde(rename = "time")]
  time: i32,
  /// Event value.
  #[serde(rename = "value")]
  value: Option<String>
}

