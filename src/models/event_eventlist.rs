

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventlist {
  /// Number of events linked to this eventgroup.
  #[serde(rename = "event_count")]
  event_count: Option<i32>,
  /// Unique identifier of eventgroup instance.
  #[serde(rename = "eventgroup_instance")]
  eventgroup_instance: Option<String>,
  /// list of all events linked to this eventgroup in chronological order.
  #[serde(rename = "events")]
  events: Option<Vec<::models::EventEventlistEvent>>,
  /// Same as eventgroup_instance.
  #[serde(rename = "id")]
  id: Option<String>
}

