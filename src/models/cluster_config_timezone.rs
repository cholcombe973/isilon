#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfigTimezone {
    /// Timezone abbreviation.
    #[serde(rename = "abbreviation")]
    pub abbreviation: Option<String>,
    /// Customer timezone information.
    #[serde(rename = "custom")]
    pub custom: Option<String>,
    /// Timezone full name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Timezone hierarchical name.
    #[serde(rename = "path")]
    pub path: Option<String>,
}
