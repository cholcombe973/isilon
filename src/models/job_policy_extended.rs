#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPolicyExtended {
    /// A helpful human-readable description of the impact policy.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "intervals")]
    pub intervals: Vec <crate::models::JobPolicyInterval>,
    /// The ID of the impact policy.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the impact policy.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether or not this is a read-only system impact policy.
    #[serde(rename = "system")]
    pub system: Option<bool>,
}
