

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Zones {
  #[serde(rename = "zones")]
  zones: Option<Vec<::models::ZoneExtended>>
}

