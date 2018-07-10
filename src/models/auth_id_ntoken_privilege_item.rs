#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthIdNtokenPrivilegeItem {
    /// Specifies the ID of the privilege.
    #[serde(rename = "id")]
    pub id: String,
    /// Specifies the name of the privilege.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// True, if the privilege is read-only.
    #[serde(rename = "read_only")]
    pub read_only: Option<bool>,
}
