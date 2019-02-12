#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpServers {
    #[serde(rename = "servers")]
    pub servers: Option<Vec <crate::models::NtpServerExtended>>,
}
