

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryQueryScopeConditions {
  #[serde(rename = "attr")]
  attr: Option<String>,
  #[serde(rename = "operator")]
  operator: Option<String>,
  #[serde(rename = "value")]
  value: Option<String>
}

