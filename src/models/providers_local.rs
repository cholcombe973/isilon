#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersLocal {
    #[serde(rename = "local")]
    pub local: Option<Vec <crate::models::ProvidersLocalLocalItem>>,
}
