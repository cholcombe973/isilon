#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusSettingsSettings {
    /// Allow access when scanning fails.
    #[serde(rename = "fail_open")]
    pub fail_open: Option<bool>,
    /// Glob patterns for leaf filenames.
    #[serde(rename = "glob_filters")]
    pub glob_filters: Option<Vec<String>>,
    /// Enable glob filters.
    #[serde(rename = "glob_filters_enabled")]
    pub glob_filters_enabled: Option<bool>,
    /// If true, only scan files matching a glob filter. If false, only scan files that don't match a glob filter.
    #[serde(rename = "glob_filters_include")]
    pub glob_filters_include: Option<bool>,
    /// Paths to include in realtime scans.
    #[serde(rename = "path_prefixes")]
    pub path_prefixes: Option<Vec<String>>,
    /// Try to quarantine files when threats are found.
    #[serde(rename = "quarantine")]
    pub quarantine: Option<bool>,
    /// Try to repair files when threats are found.
    #[serde(rename = "repair")]
    pub repair: Option<bool>,
    /// Amount of time in seconds until old reporting data is purged.
    #[serde(rename = "report_expiry")]
    pub report_expiry: Option<i32>,
    /// Scan files when apps close them.
    #[serde(rename = "scan_on_close")]
    pub scan_on_close: Option<bool>,
    /// Scan files on access.
    #[serde(rename = "scan_on_open")]
    pub scan_on_open: Option<bool>,
    /// Skip scanning files larger than this.
    #[serde(rename = "scan_size_maximum")]
    pub scan_size_maximum: Option<i32>,
    /// Whether the antivirus service is enabled.
    #[serde(rename = "service")]
    pub service: Option<bool>,
    /// Try to truncate files when threats are found.
    #[serde(rename = "truncate")]
    pub truncate: Option<bool>,
}
