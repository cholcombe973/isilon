#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpUserExtended {
    /// Unique display ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// A unique user name for NDMP administrator.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
