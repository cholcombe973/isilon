

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsVariablesVariable {
  /// The unique display id
  #[serde(rename = "id")]
  id: Option<String>,
  /// Return variables of the backup path.
  #[serde(rename = "path")]
  path: Option<String>,
  #[serde(rename = "path_variables")]
  path_variables: Option<Vec<::models::NdmpSettingsVariablesVariablePathVariable>>
}

