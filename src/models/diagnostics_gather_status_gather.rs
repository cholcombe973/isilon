#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsGatherStatusGather {
    #[serde(rename = "path")]
    pub path: Option<String>,
    ///
    #[serde(rename = "status")]
    pub status: Option <crate::models::DiagnosticsGatherStatusGatherStatus>,
}
