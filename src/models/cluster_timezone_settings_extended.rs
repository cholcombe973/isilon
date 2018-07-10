#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterTimezoneSettingsExtended {
    /// Timezone hierarchical name.
    #[serde(rename = "path")]
    pub path: Option<String>,
}
