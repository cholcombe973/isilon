

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobSummarySummary {
  /// Whether the cluster is in a degraded state.  Note this is from the perspective of the node handling the query, which might be different from another node.
  #[serde(rename = "cluster_is_degraded")]
  cluster_is_degraded: bool,
  /// Whether isi_job_d instances on all up nodes in the cluster are connected to the coordinator.
  #[serde(rename = "connected")]
  connected: bool,
  /// The devid of the job engine coordinator.
  #[serde(rename = "coordinator")]
  coordinator: i32,
  /// If connected=false, this is the set of devids that are not connected to the coordinator.
  #[serde(rename = "disconnected_nodes")]
  disconnected_nodes: Option<Vec<i32>>,
  /// Whether the cluster has any down or read-only nodes.  These nodes are not considered in the connected property.
  #[serde(rename = "down_or_read_only_nodes")]
  down_or_read_only_nodes: bool,
  /// The job ID to be assigned to the next job.
  #[serde(rename = "next_jid")]
  next_jid: i32,
  /// Whether the job engine would allow most jobs to run even when the cluster is in a degraded state.
  #[serde(rename = "run_degraded")]
  run_degraded: bool,
  /// Whether the coordinator has recently gathered statistics for all nodes in the cluster.
  #[serde(rename = "stats_ready")]
  stats_ready: bool
}

