#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChannels {
    #[serde(rename = "channels")]
    pub channels: Option<Vec<::models::EventChannelExtended>>,
}
