#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultTopFiles {
    /// Access time enabled.
    #[serde(rename = "atime_enabled")]
    pub atime_enabled: bool,
    /// Change in file ranking from result set comparison.
    #[serde(rename = "change")]
    pub change: Option<i32>,
    /// Directory access time enabled.
    #[serde(rename = "dir_atime_enabled")]
    pub dir_atime_enabled: bool,
    /// Files listing.
    #[serde(rename = "files")]
    pub files: Vec <crate::models::ResultTopFilesFile>,
    /// Limit on number of top results.
    #[serde(rename = "top_n_max")]
    pub top_n_max: i32,
    /// Total count of file results.
    #[serde(rename = "total_count")]
    pub total_count: i32,
}
