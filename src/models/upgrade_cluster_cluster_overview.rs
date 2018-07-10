

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeClusterClusterOverview {
  /// Number of nodes running the current OneFS version.
  #[serde(rename = "nodes_current")]
  nodes_current: Option<i32>,
  /// Total number of nodes on the cluster.
  #[serde(rename = "nodes_total")]
  nodes_total: Option<i32>,
  /// Number of nodes transitioning between OneFS versions. Null if the cluster_state is 'committed' or 'assessing.'
  #[serde(rename = "nodes_transitioning")]
  nodes_transitioning: Option<i32>,
  /// Number of nodes running the upgraded OneFS version. Null if the cluster_state is 'committed' or 'assessing.'
  #[serde(rename = "nodes_upgraded")]
  nodes_upgraded: Option<i32>
}

