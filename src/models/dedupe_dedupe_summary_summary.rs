

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeDedupeSummarySummary {
  /// Size in bytes of a filesystem block.
  #[serde(rename = "block_size")]
  block_size: f32,
  /// Estimated number of physical blocks deduped.
  #[serde(rename = "estimated_physical_blocks")]
  estimated_physical_blocks: f32,
  /// Estimated number of physical blocks saved by dedupe.
  #[serde(rename = "estimated_saved_blocks")]
  estimated_saved_blocks: f32,
  /// Number of logical blocks deduped.
  #[serde(rename = "logical_blocks")]
  logical_blocks: f32,
  /// Number of logical blocks saved by dedupe.
  #[serde(rename = "saved_logical_blocks")]
  saved_logical_blocks: f32,
  /// Total physical blocks in the cluster.
  #[serde(rename = "total_blocks")]
  total_blocks: f32,
  /// Total physical blocks used in the cluster.
  #[serde(rename = "used_blocks")]
  used_blocks: f32
}

