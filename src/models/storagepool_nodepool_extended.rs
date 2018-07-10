

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolNodepoolExtended {
  /// Indicates if enabling L3 is possible. L3 cannot be enabled if there are unprovisioned drives.
  #[serde(rename = "can_enable_l3")]
  can_enable_l3: bool,
  /// An array of containing any health issues with this pool.  If the pool is healthy, the list is empty.
  #[serde(rename = "health_flags")]
  health_flags: Option<Vec<String>>,
  /// The system ID given to the node pool.
  #[serde(rename = "id")]
  id: i32,
  /// Use SSDs in this node pool for L3 cache.
  #[serde(rename = "l3")]
  l3: bool,
  /// 'storage' if the 'l3' option is disabled. If the l3 option is enabled, 'migrating' if any SSDs in this node pool have not yet been migrated to L3. If all SSDs have been migrated, 'l3'.
  #[serde(rename = "l3_status")]
  l3_status: String,
  /// The nodes that are part of this node pool.
  #[serde(rename = "lnns")]
  lnns: Vec<i32>,
  /// Whether or not the node pool is manually managed.
  #[serde(rename = "manual")]
  manual: bool,
  /// The node pool name.
  #[serde(rename = "name")]
  name: String,
  /// The underlying protection policy.
  #[serde(rename = "protection_policy")]
  protection_policy: Option<String>,
  /// The name (if named) or system ID of the node pool's tier, if it is in a tier. Otherwise null.
  #[serde(rename = "tier")]
  tier: Option<String>,
  /// Total pool usage.
  #[serde(rename = "usage")]
  usage: ::models::StoragepoolTierUsage
}

