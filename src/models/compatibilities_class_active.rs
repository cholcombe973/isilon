#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesClassActive {
    #[serde(rename = "active")]
    pub active: Option<Vec<::models::CompatibilitiesClassActiveActiveItem>>,
}
