#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpUserCreateParams {
    /// The password for the NDMP administrator.
    #[serde(rename = "password")]
    pub password: String,
    /// A unique user name for NDMP administrator.
    #[serde(rename = "name")]
    pub name: String,
}
