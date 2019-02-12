#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventlistEvent {
    /// The device id of the node if it is still part of the cluster otherwise None.
    #[serde(rename = "devid")]
    pub devid: Option<i32>,
    /// Time event was ended (eventgroup resolved)
    #[serde(rename = "ended")]
    pub ended: Option<f32>,
    /// Integer identifier of the event type
    #[serde(rename = "event")]
    pub event: Option<i32>,
    /// Unique identifier of event occurrence.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The lnn of the node if it is still part of the cluster, otherwise None.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// Human readable description
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Time the event was resolved.
    #[serde(rename = "resolve_time")]
    pub resolve_time: Option<i32>,
    /// Severity of event occurrence.
    #[serde(rename = "severity")]
    pub severity: Option<String>,
    /// A collection of parameters defined per event.
    #[serde(rename = "specifier")]
    pub specifier: Option <crate::models::Empty>,
    /// Time event was detected as UNIX timestamp.
    #[serde(rename = "time")]
    pub time: Option<i32>,
    /// Value of measurement associated with this event.
    #[serde(rename = "value")]
    pub value: Option<f32>,
}
