
/// NdmpSettingsVariables : Get list of preferred environment variable.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsVariables {
  /// Resume string returned by previous query.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// The number of backup paths.
  #[serde(rename = "total")]
  total: Option<i32>,
  #[serde(rename = "variables")]
  variables: Option<Vec<::models::NdmpSettingsVariablesVariable>>
}

