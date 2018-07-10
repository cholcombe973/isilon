#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpDumpdate {
    /// The backup time since epoch.
    #[serde(rename = "date")]
    pub date: Option<i32>,
    /// The unique display id.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The level of the backup.
    #[serde(rename = "level")]
    pub level: Option<i32>,
    /// The path spedificed by NDMP environment variable FILESYSTEM.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// The snapshot ID used for a faster incremental backup. 0 means a regular backup.
    #[serde(rename = "snapid")]
    pub snapid: Option<i32>,
}
