#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsVariableCreateParams {
    /// The value of environment variable.
    #[serde(rename = "value")]
    pub value: String,
    /// The name of environment variable.
    #[serde(rename = "name")]
    pub name: String,
    /// The backup path.
    #[serde(rename = "path")]
    pub path: String,
}
