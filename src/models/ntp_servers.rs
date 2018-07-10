

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpServers {
  #[serde(rename = "servers")]
  servers: Option<Vec<::models::NtpServerExtended>>
}

