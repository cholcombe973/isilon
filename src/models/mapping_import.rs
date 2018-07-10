

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingImport {
  #[serde(rename = "identities")]
  identities: Option<Vec<Vec<String>>>
}

