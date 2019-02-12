#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsVariablesVariable {
    /// The unique display id
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Return variables of the backup path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(rename = "path_variables")]
    pub path_variables: Option<Vec <crate::models::NdmpSettingsVariablesVariablePathVariable>>,
}
