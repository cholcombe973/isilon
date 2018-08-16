#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultTopFilesFile {
    /// File access time.
    #[serde(rename = "atime")]
    pub atime: i32,
    /// File creation begin time.
    #[serde(rename = "btime")]
    pub btime: i32,
    /// Unix inode change time.
    #[serde(rename = "ctime")]
    pub ctime: i32,
    /// Logical file size in bytes.
    #[serde(rename = "log_size")]
    pub log_size: u64,
    /// Relative file path under /ifs/.
    #[serde(rename = "path")]
    pub path: String,
    /// Physical file size in bytes.
    #[serde(rename = "phys_size")]
    pub phys_size: u64,
}
