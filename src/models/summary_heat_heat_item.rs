

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryHeatHeatItem {
  /// The class of operation
  #[serde(rename = "class_name")]
  class_name: String,
  /// The type of event
  #[serde(rename = "event_name")]
  event_name: String,
  /// The event type id
  #[serde(rename = "event_type")]
  event_type: Option<i32>,
  /// Logical inode (LIN)
  #[serde(rename = "lin")]
  lin: Option<String>,
  /// The node where this event occurred.
  #[serde(rename = "node")]
  node: Option<i32>,
  /// Approximate operations per second for this lin.
  #[serde(rename = "operation_rate")]
  operation_rate: f32,
  /// Canonical LIN path if known
  #[serde(rename = "path")]
  path: String,
  /// Unix Epoch time in seconds of the request.
  #[serde(rename = "time")]
  time: i32
}

