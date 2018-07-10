
/// UpgradeCluster : Cluster wide upgrade status info.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeCluster {
  /// The cluster overview of an upgrade process.
  #[serde(rename = "cluster_overview")]
  cluster_overview: Option<::models::UpgradeClusterClusterOverview>,
  /// The different states of an upgrade, rollback, or assessment. One of the following values: 'committed', 'upgraded', 'partially upgraded', 'upgrading', 'rolling back', 'assessing', 'error'
  #[serde(rename = "cluster_state")]
  cluster_state: Option<String>,
  /// The current upgrade activity.
  #[serde(rename = "current_process")]
  current_process: Option<String>,
  /// The time when a rollback, assessment or upgrade has finished completely. Use ISO 8601 standard. Null if the cluster_state is not 'upgraded'.
  #[serde(rename = "finish_time")]
  finish_time: Option<String>,
  /// The location (path) of the upgrade image which must be within /ifs. Null if the cluster_state is 'committed' or 'upgraded.'
  #[serde(rename = "install_image_path")]
  install_image_path: Option<String>,
  /// The median time (seconds) to complete each node so far during this upgrade. Before the first node in an upgrade has completed this key will have an associated null value.
  #[serde(rename = "node_median_time")]
  node_median_time: Option<i32>,
  /// The current OneFS version before upgrade.
  #[serde(rename = "onefs_version_current")]
  onefs_version_current: Option<::models::ClusterNodesOnefsVersion>,
  /// The OneFS version the user is attempting to upgrade to. Null if the cluster_state is 'committed' or 'assessing.'
  #[serde(rename = "onefs_version_upgrade")]
  onefs_version_upgrade: Option<::models::ClusterNodesOnefsVersion>,
  /// The most recent patch action performed.
  #[serde(rename = "patch_action")]
  patch_action: Option<String>,
  /// The patch with the most recent patch action.
  #[serde(rename = "patch_name")]
  patch_name: Option<String>,
  /// The time when an upgrade, rollback, or assessment was started. Use ISO 8601 standard. Null if the cluster_state is 'committed' or 'partially upgraded.'
  #[serde(rename = "start_time")]
  start_time: Option<String>,
  /// True if upgrade is committed.
  #[serde(rename = "upgrade_is_committed")]
  upgrade_is_committed: Option<bool>,
  /// The settings necessary when starting an upgrade. Null if the cluster_state is not 'upgrading' or 'partially upgraded.' or 'error'.
  #[serde(rename = "upgrade_settings")]
  upgrade_settings: Option<::models::UpgradeClusterUpgradeSettings>,
  /// Time at which upgrade was originally requested.
  #[serde(rename = "upgrade_triggered_time")]
  upgrade_triggered_time: Option<String>
}

