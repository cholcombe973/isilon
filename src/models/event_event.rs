/// EventEvent : Test Event Specifier

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEvent {
    /// Message for test event
    #[serde(rename = "message")]
    pub message: Option<String>,
}
