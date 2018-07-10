#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsGlobalGlobal {
    /// Maximum number of BRE contexts.
    #[serde(rename = "bre_max_num_contexts")]
    pub bre_max_num_contexts: Option<i32>,
    /// A unique identifier for the dma vendor.
    #[serde(rename = "dma")]
    pub dma: Option<String>,
    /// Multi-Stream Backup context retention duration.
    #[serde(rename = "msb_context_retention_duration")]
    pub msb_context_retention_duration: Option<i32>,
    /// Multi-Stream Restore context retention duration.
    #[serde(rename = "msr_context_retention_duration")]
    pub msr_context_retention_duration: Option<i32>,
    /// The port to listen on.
    #[serde(rename = "port")]
    pub port: Option<i32>,
    /// Property to enable/diable the NDMP service.
    #[serde(rename = "service")]
    pub service: Option<bool>,
}
