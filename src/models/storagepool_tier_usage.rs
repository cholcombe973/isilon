#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolTierUsage {
    /// Available free bytes remaining in the pool when virtual hot spare is taken into account.
    #[serde(rename = "avail_bytes")]
    pub avail_bytes: String,
    /// Available free bytes remaining in the pool on HDD drives when virtual hot spare is taken into account.
    #[serde(rename = "avail_hdd_bytes")]
    pub avail_hdd_bytes: String,
    /// Available free bytes remaining in the pool on SSD drives when virtual hot spare is taken into account.
    #[serde(rename = "avail_ssd_bytes")]
    pub avail_ssd_bytes: String,
    /// Whether or not the pool usage is currently balanced.
    #[serde(rename = "balanced")]
    pub balanced: bool,
    /// Free bytes remaining in the pool.
    #[serde(rename = "free_bytes")]
    pub free_bytes: String,
    /// Free bytes remaining in the pool on HDD drives.
    #[serde(rename = "free_hdd_bytes")]
    pub free_hdd_bytes: String,
    /// Free bytes remaining in the pool on SSD drives.
    #[serde(rename = "free_ssd_bytes")]
    pub free_ssd_bytes: String,
    /// Percentage of usable space in the pool which is used.
    #[serde(rename = "pct_used")]
    pub pct_used: String,
    /// Percentage of usable space on HDD drives in the pool which is used.
    #[serde(rename = "pct_used_hdd")]
    pub pct_used_hdd: String,
    /// Percentage of usable space on SSD drives in the pool which is used.
    #[serde(rename = "pct_used_ssd")]
    pub pct_used_ssd: String,
    /// Total bytes in the pool.
    #[serde(rename = "total_bytes")]
    pub total_bytes: String,
    /// Total bytes in the pool on HDD drives.
    #[serde(rename = "total_hdd_bytes")]
    pub total_hdd_bytes: String,
    /// Total bytes in the pool on SSD drives.
    #[serde(rename = "total_ssd_bytes")]
    pub total_ssd_bytes: String,
    /// Total bytes in the pool drives when virtual hot spare is taken into account.
    #[serde(rename = "usable_bytes")]
    pub usable_bytes: String,
    /// Total bytes in the pool on HDD drives when virtual hot spare is taken into account.
    #[serde(rename = "usable_hdd_bytes")]
    pub usable_hdd_bytes: String,
    /// Total bytes in the pool on SSD drives when virtual hot spare is taken into account.
    #[serde(rename = "usable_ssd_bytes")]
    pub usable_ssd_bytes: String,
    /// Used bytes in the pool.
    #[serde(rename = "used_bytes")]
    pub used_bytes: Option<String>,
    /// Used bytes in the pool on HDD drives.
    #[serde(rename = "used_hdd_bytes")]
    pub used_hdd_bytes: String,
    /// Used bytes in the pool on SSD drives.
    #[serde(rename = "used_ssd_bytes")]
    pub used_ssd_bytes: String,
    #[serde(rename = "virtual_hot_spare_bytes")]
    pub virtual_hot_spare_bytes: Option<String>,
}
