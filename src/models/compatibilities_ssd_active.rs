#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdActive {
    #[serde(rename = "active")]
    pub active: Option<Vec <crate::models::CompatibilitiesSsdActiveActiveItem>>,
}
