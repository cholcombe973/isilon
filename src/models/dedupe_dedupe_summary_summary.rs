#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeDedupeSummarySummary {
    /// Size in bytes of a filesystem block.
    #[serde(rename = "block_size")]
    pub block_size: u64,
    /// Estimated number of physical blocks deduped.
    #[serde(rename = "estimated_physical_blocks")]
    pub estimated_physical_blocks: f32,
    /// Estimated number of physical blocks saved by dedupe.
    #[serde(rename = "estimated_saved_blocks")]
    pub estimated_saved_blocks: f32,
    /// Number of logical blocks deduped.
    #[serde(rename = "logical_blocks")]
    pub logical_blocks: f32,
    /// Number of logical blocks saved by dedupe.
    #[serde(rename = "saved_logical_blocks")]
    pub saved_logical_blocks: f32,
    /// Total physical blocks in the cluster.
    #[serde(rename = "total_blocks")]
    pub total_blocks: f32,
    /// Total physical blocks used in the cluster.
    #[serde(rename = "used_blocks")]
    pub used_blocks: f32,
}
