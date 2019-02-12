#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStoragepool {
    /// Indicates if disabling L3 is possible.
    #[serde(rename = "can_disable_l3")]
    pub can_disable_l3: Option<bool>,
    /// Indicates if enabling L3 is possible. L3 cannot be enabled if there are unprovisioned drives.
    #[serde(rename = "can_enable_l3")]
    pub can_enable_l3: Option<bool>,
    /// The names or IDs of the pool's children.
    #[serde(rename = "children")]
    pub children: Option<Vec<String>>,
    /// An array of containing any health issues with this pool.  If the pool is healthy, the list is empty.  Only appears on 'nodepool' type storagepools.
    #[serde(rename = "health_flags")]
    pub health_flags: Option<Vec<String>>,
    /// The system ID given to the pool.
    #[serde(rename = "id")]
    pub id: i32,
    /// Use SSDs in this node pool for L3 cache.
    #[serde(rename = "l3")]
    pub l3: Option<bool>,
    /// 'storage' if the 'l3' option is disabled. If the l3 option is enabled, 'migrating' if any SSDs in this node pool have not yet been migrated to L3. If all SSDs have been migrated, 'l3'.
    #[serde(rename = "l3_status")]
    pub l3_status: Option<String>,
    /// The nodes that are part of this pool.
    #[serde(rename = "lnns")]
    pub lnns: Vec<i32>,
    /// Whether or not the node pool is manually managed.
    #[serde(rename = "manual")]
    pub manual: Option<bool>,
    /// The pool name.
    #[serde(rename = "name")]
    pub name: String,
    /// The underlying protection policy.
    #[serde(rename = "protection_policy")]
    pub protection_policy: Option<String>,
    /// The pool type.
    #[serde(rename = "type")]
    pub _type: String,
    /// Total pool usage.
    #[serde(rename = "usage")]
    pub usage:crate::models::StoragepoolTierUsage,
}
