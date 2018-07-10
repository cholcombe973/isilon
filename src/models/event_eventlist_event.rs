

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventlistEvent {
  /// The device id of the node if it is still part of the cluster otherwise None.
  #[serde(rename = "devid")]
  devid: Option<i32>,
  /// Time event was ended (eventgroup resolved)
  #[serde(rename = "ended")]
  ended: Option<f32>,
  /// Integer identifier of the event type
  #[serde(rename = "event")]
  event: Option<i32>,
  /// Unique identifier of event occurrence.
  #[serde(rename = "id")]
  id: Option<String>,
  /// The lnn of the node if it is still part of the cluster, otherwise None.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// Human readable description
  #[serde(rename = "message")]
  message: Option<String>,
  /// Time the event was resolved.
  #[serde(rename = "resolve_time")]
  resolve_time: Option<i32>,
  /// Severity of event occurrence.
  #[serde(rename = "severity")]
  severity: Option<String>,
  /// A collection of parameters defined per event.
  #[serde(rename = "specifier")]
  specifier: Option<::models::Empty>,
  /// Time event was detected as UNIX timestamp.
  #[serde(rename = "time")]
  time: Option<i32>,
  /// Value of measurement associated with this event.
  #[serde(rename = "value")]
  value: Option<f32>
}

