

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersNis {
  #[serde(rename = "nis")]
  nis: Option<Vec<::models::ProvidersNisNisItem>>
}

