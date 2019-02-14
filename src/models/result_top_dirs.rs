#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultTopDirs {
    /// Change in directory ranking from result set comparison.
    #[serde(rename = "change")]
    pub change: Option<i32>,
    /// Directory access time enabled.
    #[serde(rename = "dir_atime_enabled")]
    pub dir_atime_enabled: bool,
    /// Directory listing.
    #[serde(rename = "dirs")]
    pub dirs: Vec <crate::models::ResultTopDirsDir>,
    /// Limit on number of top results.
    #[serde(rename = "top_n_max")]
    pub top_n_max: i32,
    /// Total count of directory results.
    #[serde(rename = "total_count")]
    pub total_count: i32,
}
