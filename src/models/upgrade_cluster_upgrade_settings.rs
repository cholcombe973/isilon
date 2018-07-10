

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeClusterUpgradeSettings {
  /// The nodes (to be) scheduled for upgrade ordered by queue position number. Null if the cluster_state is 'partially upgraded' or upgrade_type is 'simultaneous'. One of the following values: [<lnn-1>, <lnn-2>, ... ], 'All', null
  #[serde(rename = "nodes_to_rolling_upgrade")]
  nodes_to_rolling_upgrade: Option<Vec<i32>>,
  /// The type of upgrade to perform. One of the following values: 'rolling', 'simultaneous'
  #[serde(rename = "upgrade_type")]
  upgrade_type: Option<String>
}

