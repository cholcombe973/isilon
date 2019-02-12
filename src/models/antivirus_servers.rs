#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusServers {
    #[serde(rename = "servers")]
    pub servers: Option<Vec <crate::models::AntivirusServerExtended>>,
}
