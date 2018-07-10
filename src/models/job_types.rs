

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobTypes {
  #[serde(rename = "types")]
  types: Option<Vec<::models::JobTypeExtended>>
}

