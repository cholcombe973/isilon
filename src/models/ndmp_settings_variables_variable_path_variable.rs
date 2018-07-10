

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsVariablesVariablePathVariable {
  /// The name of environment variable.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The value of environment variable.
  #[serde(rename = "value")]
  value: Option<String>
}

