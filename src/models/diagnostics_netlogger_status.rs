#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsNetloggerStatus {
    ///
    #[serde(rename = "netlogger")]
    pub netlogger: Option <crate::models::DiagnosticsGatherStatusGather>,
}
