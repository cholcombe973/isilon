#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeIo {
    ///
    #[serde(rename = "read")]
    pub read:crate::models::JobStatisticsJobNodeIoRead,
    ///
    #[serde(rename = "write")]
    pub write:crate::models::JobStatisticsJobNodeIoWrite,
}
