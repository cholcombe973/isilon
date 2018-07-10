#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbShares {
    #[serde(rename = "shares")]
    pub shares: Option<Vec<::models::SmbShareExtended>>,
}
