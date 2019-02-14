#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolUnprovisionedUnprovisioned {
    /// A list of unprovisioned drives that do not belong to an unprovisioned node.
    #[serde(rename = "drives")]
    pub drives: Vec <crate::models::StoragepoolStatusUnprovisionedItem>,
    /// A list of lnns whose drives are all unprovisioned
    #[serde(rename = "lnns")]
    pub lnns: Vec<i32>,
}
