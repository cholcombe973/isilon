

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultDirectories {
  /// User attribute count.
  #[serde(rename = "attribute_count")]
  attribute_count: i32,
  /// Directory depth.
  #[serde(rename = "dir_depth")]
  dir_depth: i32,
  /// Disk usage for current directory.
  #[serde(rename = "dir_usage")]
  dir_usage: ::models::ResultDirectoriesTotalUsage,
  /// Directory path information from root to current directory.
  #[serde(rename = "path_parts")]
  path_parts: Vec<String>,
  /// Disk usage from root.
  #[serde(rename = "total_usage")]
  total_usage: ::models::ResultDirectoriesTotalUsage,
  /// Disk usage for all of immediate children of the current directory.
  #[serde(rename = "usage_data")]
  usage_data: Vec<::models::ResultDirectoriesUsageDataItem>
}

