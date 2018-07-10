/// DedupeSettingsExtended : Dedupe settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeSettingsExtended {
    /// The paths that will be assessed.
    #[serde(rename = "assess_paths")]
    pub assess_paths: Option<Vec<String>>,
    /// The paths that will be deduped.
    #[serde(rename = "paths")]
    pub paths: Option<Vec<String>>,
}
