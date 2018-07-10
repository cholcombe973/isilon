
/// ClusterStatfs : Filesystem statistics

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterStatfs {
  /// The number of free blocks available to non-superuser.
  #[serde(rename = "f_bavail")]
  f_bavail: u64,
  /// The number of free blocks in the filesystem.
  #[serde(rename = "f_bfree")]
  f_bfree: u64,
  /// The total number of data blocks in the filesystem.
  #[serde(rename = "f_blocks")]
  f_blocks: u64,
  /// The filesystem fragment size.
  #[serde(rename = "f_bsize")]
  f_bsize: u64,
  /// The number of free nodes available to non-superuser.
  #[serde(rename = "f_ffree")]
  f_ffree: u64,
  /// The total number of file nodes in the filesystem.
  #[serde(rename = "f_files")]
  f_files: u64,
  /// A copy of the mount exported flags.
  #[serde(rename = "f_flags")]
  f_flags: u64,
  /// The filesystem type name.
  #[serde(rename = "f_fstypename")]
  f_fstypename: String,
  /// The optimal transfer block size.
  #[serde(rename = "f_iosize")]
  f_iosize: u64,
  /// The name of the mounted filesystem.
  #[serde(rename = "f_mntfromname")]
  f_mntfromname: String,
  /// The directory that the filesystem is mounted on.
  #[serde(rename = "f_mntonname")]
  f_mntonname: String,
  /// The maximum length of a file name.
  #[serde(rename = "f_namemax")]
  f_namemax: u64,
  /// The ID of the user that mounted the filesystem.
  #[serde(rename = "f_owner")]
  f_owner: u64,
  /// The type of the filesystem.
  #[serde(rename = "f_type")]
  f_type: u64,
  /// The structure version number.
  #[serde(rename = "f_version")]
  f_version: u64
}

