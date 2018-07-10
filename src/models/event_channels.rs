

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChannels {
  #[serde(rename = "channels")]
  channels: Option<Vec<::models::EventChannelExtended>>
}

