/// SettingsAclsExtended : ACL policies settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAclsExtended {
    /// Access checks (chmod, chown).
    #[serde(rename = "access")]
    pub access: Option<String>,
    /// Displayed mode bits.
    #[serde(rename = "calcmode")]
    pub calcmode: Option<String>,
    /// Approximate group mode bits when ACL exists.
    #[serde(rename = "calcmode_group")]
    pub calcmode_group: Option<String>,
    /// Approximate owner mode bits when ACL exists.
    #[serde(rename = "calcmode_owner")]
    pub calcmode_owner: Option<String>,
    /// chmod on files with existing ACLs.
    #[serde(rename = "chmod")]
    pub chmod: Option<String>,
    /// chmod (007) on files with existing ACLs.
    #[serde(rename = "chmod_007")]
    pub chmod_007: Option<String>,
    /// ACLs created on directories by UNIX chmod.
    #[serde(rename = "chmod_inheritable")]
    pub chmod_inheritable: Option<String>,
    /// chown/chgrp on files with existing ACLs.
    #[serde(rename = "chown")]
    pub chown: Option<String>,
    /// ACL creation over SMB.
    #[serde(rename = "create_over_smb")]
    pub create_over_smb: Option<String>,
    ///  Read only DOS attribute.
    #[serde(rename = "dos_attr")]
    pub dos_attr: Option<String>,
    /// Group owner inheritance.
    #[serde(rename = "group_owner_inheritance")]
    pub group_owner_inheritance: Option<String>,
    /// Treatment of 'rwx' permissions.
    #[serde(rename = "rwx")]
    pub rwx: Option<String>,
    /// Synthetic 'deny' ACEs.
    #[serde(rename = "synthetic_denies")]
    pub synthetic_denies: Option<String>,
    /// Access check (utimes)
    #[serde(rename = "utimes")]
    pub utimes: Option<String>,
}
