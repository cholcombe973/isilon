#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterTimezoneSettings {
    /// Timezone abbreviation.
    #[serde(rename = "abbreviation")]
    pub abbreviation: Option<String>,
    /// Timezone hierarchical name.
    #[serde(rename = "path")]
    pub path: Option<String>,
}
