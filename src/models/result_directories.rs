#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultDirectories {
    /// User attribute count.
    #[serde(rename = "attribute_count")]
    pub attribute_count: i32,
    /// Directory depth.
    #[serde(rename = "dir_depth")]
    pub dir_depth: i32,
    /// Disk usage for current directory.
    #[serde(rename = "dir_usage")]
    pub dir_usage: ::models::ResultDirectoriesTotalUsage,
    /// Directory path information from root to current directory.
    #[serde(rename = "path_parts")]
    pub path_parts: Vec<String>,
    /// Disk usage from root.
    #[serde(rename = "total_usage")]
    pub total_usage: ::models::ResultDirectoriesTotalUsage,
    /// Disk usage for all of immediate children of the current directory.
    #[serde(rename = "usage_data")]
    pub usage_data: Vec<::models::ResultDirectoriesUsageDataItem>,
}
