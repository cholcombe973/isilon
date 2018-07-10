#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPrivilege {
    /// Specifies the general categorization of the privilege.
    #[serde(rename = "category")]
    pub category: String,
    /// Specifies a short description of the privilege.
    #[serde(rename = "description")]
    pub description: String,
    /// Specifies the ID of the privilege.
    #[serde(rename = "id")]
    pub id: String,
    /// Specifies the name of the privilege.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// True, if the privilege is read-write.
    #[serde(rename = "read_write")]
    pub read_write: Option<bool>,
}
