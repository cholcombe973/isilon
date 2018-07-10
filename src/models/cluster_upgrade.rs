
/// ClusterUpgrade : Add nodes to a running upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterUpgrade {
  /// The nodes (to be) scheduled for an existing upgrade ordered by queue position number. [<lnn-1>, <lnn-2>, ... ], 'All', null
  #[serde(rename = "nodes_to_rolling_upgrade")]
  nodes_to_rolling_upgrade: Option<Vec<i32>>
}

