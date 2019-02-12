/// MappingIdentityCreateParams : Specifies the properties for the identity mapping entry.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingIdentityCreateParams {
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "source")]
    pub source:crate::models::AuthAccessAccessItemFileGroup,
    #[serde(rename = "targets")]
    pub targets: Vec <crate::models::MappingIdentityTargetCreateParams>,
}
