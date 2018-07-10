#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSettingsSettings {
    /// True if the scheduled snapshot creation services is on.
    #[serde(rename = "autocreate")]
    pub autocreate: bool,
    /// True if the scheduled snapshot deletion services is on.
    #[serde(rename = "autodelete")]
    pub autodelete: bool,
    /// Global switch for other accessible and visible settings.
    #[serde(rename = "global_visible_accessible")]
    pub global_visible_accessible: bool,
    /// True if root .snapshot directory is accessible locally.
    #[serde(rename = "local_root_accessible")]
    pub local_root_accessible: bool,
    /// True if root .snapshot directory is visible locally.
    #[serde(rename = "local_root_visible")]
    pub local_root_visible: bool,
    /// True if sub-directory .snapshot directory is accessible locally.
    #[serde(rename = "local_subdir_accessible")]
    pub local_subdir_accessible: bool,
    /// True if root .snapshot directory is accessible over NFS.
    #[serde(rename = "nfs_root_accessible")]
    pub nfs_root_accessible: bool,
    /// True if root .snapshot directory is visible over NFS.
    #[serde(rename = "nfs_root_visible")]
    pub nfs_root_visible: bool,
    /// True if sub-directory .snapshot directory is accessible over NFS.
    #[serde(rename = "nfs_subdir_accessible")]
    pub nfs_subdir_accessible: bool,
    /// Percentage of space to reserve for snapshots.
    #[serde(rename = "reserve")]
    pub reserve: f32,
    /// True if the system allows snapshot creation.
    #[serde(rename = "service")]
    pub service: bool,
    /// True if root .snapshot directory is accessible over SMB.
    #[serde(rename = "smb_root_accessible")]
    pub smb_root_accessible: bool,
    /// True if root .snapshot directory is visible over SMB.
    #[serde(rename = "smb_root_visible")]
    pub smb_root_visible: bool,
    /// True if sub-directory .snapshot directory is accessible over SMB.
    #[serde(rename = "smb_subdir_accessible")]
    pub smb_subdir_accessible: bool,
}
