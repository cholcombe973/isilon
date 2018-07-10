
/// ClusterRetryLastActionItem : Retry the last upgrade action, in-case the previous attempt failed.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterRetryLastActionItem {
  /// List of the nodes or \"all\" where the last upgrade action can be retried.
  #[serde(rename = "nodes")]
  nodes: Option<Vec<i32>>
}

