#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersNis {
    #[serde(rename = "nis")]
    pub nis: Option<Vec <crate::models::ProvidersNisNisItem>>,
}
