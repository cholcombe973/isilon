#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareStatusStatusItem {
    /// The ID of this upgrade.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the upgrading pool.
    #[serde(rename = "nodepool_name")]
    pub nodepool_name: Option<String>,
    /// The lnns of the nodes in the pool that haven't been upgraded yet.
    #[serde(rename = "unupgraded_lnns")]
    pub unupgraded_lnns: Vec<i32>,
    /// The type of upgrade this is.
    #[serde(rename = "upgrade_type")]
    pub upgrade_type: String,
    /// The lnns of the nodes in the pool that have been successsfully upgraded.
    #[serde(rename = "upgraded_lnns")]
    pub upgraded_lnns: Vec<i32>,
}
