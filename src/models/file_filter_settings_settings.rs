

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFilterSettingsSettings {
  /// List of file extensions to be filtered.
  #[serde(rename = "file_filter_extensions")]
  file_filter_extensions: Option<Vec<String>>,
  /// Specifies if filter list is for deny or allow. Default is deny.
  #[serde(rename = "file_filter_type")]
  file_filter_type: Option<String>,
  /// Indicates whether file filtering is enabled on this zone.
  #[serde(rename = "file_filtering_enabled")]
  file_filtering_enabled: Option<bool>
}

