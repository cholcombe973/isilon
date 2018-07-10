#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBreEnvVariable {
    /// Environment variable name
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Environment variable value
    #[serde(rename = "value")]
    pub value: Option<String>,
}
