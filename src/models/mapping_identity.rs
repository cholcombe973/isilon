#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingIdentity {
    /// Specifies the identity mapping entry id.
    #[serde(rename = "id")]
    pub id: String,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "source")]
    pub source: Option <crate::models::AuthAccessAccessItemFileGroup>,
    #[serde(rename = "targets")]
    pub targets: Vec <crate::models::MappingIdentityTarget>,
}
