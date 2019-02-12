#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingIdentities {
    #[serde(rename = "identities")]
    pub identities: Option<Vec <crate::models::MappingIdentity>>,
}
