#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolRuleCreateParams {
    /// Description for the provisioning rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Interface name the provisioning rule applies to.
    #[serde(rename = "iface")]
    pub iface: String,
    /// Name of the provisioning rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Node type the provisioning rule applies to.
    #[serde(rename = "node_type")]
    pub node_type: Option<String>,
}
