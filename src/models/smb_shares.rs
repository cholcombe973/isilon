

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbShares {
  #[serde(rename = "shares")]
  shares: Option<Vec<::models::SmbShareExtended>>
}

