/// ClusterArchiveItem : Start an archive of an upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterArchiveItem {
    /// If set to true the currently running upgrade will be cleared
    #[serde(rename = "clear")]
    pub clear: Option<bool>,
}
