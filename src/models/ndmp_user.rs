#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpUser {
    /// The password for the NDMP administrator.
    #[serde(rename = "password")]
    pub password: String,
}
