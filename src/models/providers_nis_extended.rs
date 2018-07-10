

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersNisExtended {
  #[serde(rename = "nis")]
  nis: Option<Vec<::models::ProvidersNisNisItemExtended>>
}

