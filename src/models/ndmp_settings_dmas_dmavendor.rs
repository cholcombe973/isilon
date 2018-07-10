#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsDmasDmavendor {
    /// NDMP dma vendor.
    #[serde(rename = "dmavendor")]
    pub dmavendor: Option<String>,
}
