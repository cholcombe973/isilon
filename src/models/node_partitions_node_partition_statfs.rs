#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodePartitionsNodePartitionStatfs {
    /// Free blocks available to non-superuser on this partition.
    #[serde(rename = "f_bavail")]
    pub f_bavail: Option<i32>,
    /// Free blocks on this partition.
    #[serde(rename = "f_bfree")]
    pub f_bfree: Option<i32>,
    /// Total data blocks on this partition.
    #[serde(rename = "f_blocks")]
    pub f_blocks: Option<i32>,
    /// Filesystem fragment size; block size in OneFS.
    #[serde(rename = "f_bsize")]
    pub f_bsize: Option<i32>,
    /// Free file nodes avail to non-superuser.
    #[serde(rename = "f_ffree")]
    pub f_ffree: Option<i32>,
    /// Total file nodes in filesystem.
    #[serde(rename = "f_files")]
    pub f_files: Option<i32>,
    /// Mount exported flags.
    #[serde(rename = "f_flags")]
    pub f_flags: Option<i32>,
    /// File system type name.
    #[serde(rename = "f_fstypename")]
    pub f_fstypename: Option<String>,
    /// Optimal transfer block size.
    #[serde(rename = "f_iosize")]
    pub f_iosize: Option<i32>,
    /// Names of devices this partition is mounted from.
    #[serde(rename = "f_mntfromname")]
    pub f_mntfromname: Option<String>,
    /// Directory this partition is mounted to.
    #[serde(rename = "f_mntonname")]
    pub f_mntonname: Option<String>,
    /// Maximum filename length.
    #[serde(rename = "f_namemax")]
    pub f_namemax: Option<i32>,
    /// UID of user that mounted the filesystem.
    #[serde(rename = "f_owner")]
    pub f_owner: Option<i32>,
    /// Type of filesystem.
    #[serde(rename = "f_type")]
    pub f_type: Option<i32>,
    /// statfs() structure version number.
    #[serde(rename = "f_version")]
    pub f_version: Option<i32>,
}
