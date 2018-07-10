#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFilterSettingsExtended {
    /// List of file extensions to be filtered.
    #[serde(rename = "file_filter_extensions")]
    pub file_filter_extensions: Option<Vec<String>>,
    /// Specifies if filter list is for deny or allow. Default is deny.
    #[serde(rename = "file_filter_type")]
    pub file_filter_type: Option<String>,
    /// Indicates whether file filtering is enabled on this zone.
    #[serde(rename = "file_filtering_enabled")]
    pub file_filtering_enabled: Option<bool>,
}
