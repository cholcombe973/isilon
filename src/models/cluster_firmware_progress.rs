
/// ClusterFirmwareProgress : Cluster wide firmware upgrade progress info.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFirmwareProgress {
  /// The different states of a  firmware upgrade. One of the following values: 'committed', 'upgrading', 'error'
  #[serde(rename = "cluster_state")]
  cluster_state: Option<String>
}

