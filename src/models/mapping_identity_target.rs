#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingIdentityTarget {
    /// If true, the identity is preferred on-disk.
    #[serde(rename = "on_disk")]
    pub on_disk: Option<bool>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "target")]
    pub target: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies the origin of the identity mapping.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
