

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupOccurrencesEventgroup {
  /// List of eventgroup IDs that may be causes of this occurrence.
  #[serde(rename = "causes")]
  causes: Option<Vec<Vec<String>>>,
  /// List of channels to which alerts are configured for this eventgroup
  #[serde(rename = "channels")]
  channels: Option<Vec<String>>,
  /// Number of events linked to this eventgroup.
  #[serde(rename = "event_count")]
  event_count: Option<i32>,
  /// Unique identifier of eventgroup instance.
  #[serde(rename = "eventgroup_instance")]
  eventgroup_instance: Option<String>,
  /// Same as eventgroup_instance.
  #[serde(rename = "id")]
  id: Option<String>,
  /// True if eventgroup is marked as 'ignore'.
  #[serde(rename = "ignore")]
  ignore: Option<bool>,
  /// Time eventgroup was marked as 'ignore'.
  #[serde(rename = "ignore_time")]
  ignore_time: Option<i32>,
  /// Time the most recent event was added to this eventgroup.
  #[serde(rename = "last_event")]
  last_event: Option<i32>,
  /// When the eventgroup became resolved.
  #[serde(rename = "resolve_time")]
  resolve_time: Option<i32>,
  /// True if eventgroup is resolved.
  #[serde(rename = "resolved")]
  resolved: Option<bool>,
  /// 'USER' if the eventgroup was marked resolved via PAPI <event_instance> if eventgroup was marked resolved as a result of an event.
  #[serde(rename = "resolver")]
  resolver: Option<String>,
  /// XXX description needed.
  #[serde(rename = "sequence")]
  sequence: Option<i32>,
  /// Event Group severity.
  #[serde(rename = "severity")]
  severity: Option<String>,
  /// A collection of parameters defined per eventgroup_id.
  #[serde(rename = "specifier")]
  specifier: Option<::models::Empty>,
  /// Time of first event linked to this eventgroup.
  #[serde(rename = "time_noticed")]
  time_noticed: Option<i32>
}

