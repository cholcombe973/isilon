#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsGatherStatus {
    ///
    #[serde(rename = "gather")]
    pub gather: Option <crate::models::DiagnosticsGatherStatusGather>,
}
