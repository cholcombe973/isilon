#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolUnprovisioned {
    ///
    #[serde(rename = "unprovisioned")]
    pub unprovisioned: Option <crate::models::StoragepoolUnprovisionedUnprovisioned>,
}
