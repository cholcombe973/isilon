#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolRulesRule {
    /// Description for the provisioning rule.
    #[serde(rename = "description")]
    pub description: String,
    /// Name of the groupnet this rule belongs to
    #[serde(rename = "groupnet")]
    pub groupnet: String,
    /// Unique rule ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Interface name the provisioning rule applies to.
    #[serde(rename = "iface")]
    pub iface: String,
    /// Name of the provisioning rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Node type the provisioning rule applies to.
    #[serde(rename = "node_type")]
    pub node_type: String,
    /// Name of the pool this rule belongs to.
    #[serde(rename = "pool")]
    pub pool: String,
    /// Name of the subnet this rule belongs to.
    #[serde(rename = "subnet")]
    pub subnet: String,
}
