#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolNodepoolCreateParams {
    /// Use SSDs in this node pool for L3 cache.
    #[serde(rename = "l3")]
    pub l3: Option<bool>,
    /// The nodes that are part of this node pool.
    #[serde(rename = "lnns")]
    pub lnns: Vec<i32>,
    /// The node pool name.
    #[serde(rename = "name")]
    pub name: String,
    /// The node pool protection policy.
    #[serde(rename = "protection_policy")]
    pub protection_policy: Option<String>,
    /// The name or ID of the node pool's tier, if it is in a tier.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
}
