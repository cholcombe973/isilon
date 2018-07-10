

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolRule {
  /// Description for the provisioning rule.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Interface name the provisioning rule applies to.
  #[serde(rename = "iface")]
  iface: Option<String>,
  /// Name of the provisioning rule.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Node type the provisioning rule applies to.
  #[serde(rename = "node_type")]
  node_type: Option<String>
}

