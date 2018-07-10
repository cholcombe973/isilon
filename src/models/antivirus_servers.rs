

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusServers {
  #[serde(rename = "servers")]
  servers: Option<Vec<::models::AntivirusServerExtended>>
}

