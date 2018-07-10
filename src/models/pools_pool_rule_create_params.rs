

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolRuleCreateParams {
  /// Description for the provisioning rule.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Interface name the provisioning rule applies to.
  #[serde(rename = "iface")]
  iface: String,
  /// Name of the provisioning rule.
  #[serde(rename = "name")]
  name: String,
  /// Node type the provisioning rule applies to.
  #[serde(rename = "node_type")]
  node_type: Option<String>
}

