

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryQueryScope {
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::DirectoryQueryScopeConditions>>,
  #[serde(rename = "logic")]
  logic: Option<String>
}

