

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolRulesRule {
  /// Description for the provisioning rule.
  #[serde(rename = "description")]
  description: String,
  /// Name of the groupnet this rule belongs to
  #[serde(rename = "groupnet")]
  groupnet: String,
  /// Unique rule ID.
  #[serde(rename = "id")]
  id: String,
  /// Interface name the provisioning rule applies to.
  #[serde(rename = "iface")]
  iface: String,
  /// Name of the provisioning rule.
  #[serde(rename = "name")]
  name: String,
  /// Node type the provisioning rule applies to.
  #[serde(rename = "node_type")]
  node_type: String,
  /// Name of the pool this rule belongs to.
  #[serde(rename = "pool")]
  pool: String,
  /// Name of the subnet this rule belongs to.
  #[serde(rename = "subnet")]
  subnet: String
}

