/// ClusterUpgradeItem : The settings necessary to start an upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterUpgradeItem {
    /// The location (path) of the upgrade image which must be within /ifs.
    #[serde(rename = "install_image_path")]
    pub install_image_path: Option<String>,
    /// The nodes (to be) scheduled for upgrade ordered by queue position number. Null if the cluster_state is 'partially upgraded' or upgrade_type is 'simultaneous'. One of the following values: [<lnn-1>, <lnn-2>, ... ], 'All', null
    #[serde(rename = "nodes_to_rolling_upgrade")]
    pub nodes_to_rolling_upgrade: Option<Vec<i32>>,
    /// Used to indicate that the pre-upgrade check should be skipped
    #[serde(rename = "skip_optional")]
    pub skip_optional: Option<bool>,
    /// The type of upgrade to perform. One of the following values: 'rolling', 'simultaneous'
    #[serde(rename = "upgrade_type")]
    pub upgrade_type: Option<String>,
}
