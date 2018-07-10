#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPolicyCreateParams {
    /// A helpful human-readable description of the impact policy.
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "intervals")]
    pub intervals: Option<Vec<::models::JobPolicyInterval>>,
    /// The name of the impact policy.
    #[serde(rename = "name")]
    pub name: String,
}
