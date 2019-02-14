/// NdmpSettingsVariables : Get list of preferred environment variable.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsVariables {
    /// Resume string returned by previous query.
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// The number of backup paths.
    #[serde(rename = "total")]
    pub total: Option<i32>,
    #[serde(rename = "variables")]
    pub variables: Option<Vec <crate::models::NdmpSettingsVariablesVariable>>,
}
