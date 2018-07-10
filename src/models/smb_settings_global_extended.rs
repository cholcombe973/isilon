#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSettingsGlobalExtended {
    /// Only enumerate files and folders the requesting user has access to.
    #[serde(rename = "access_based_share_enum")]
    pub access_based_share_enum: Option<bool>,
    /// Specify level of file share audit events to log.
    #[serde(rename = "audit_fileshare")]
    pub audit_fileshare: Option<String>,
    /// Specifies a list of permissions to audit.
    #[serde(rename = "audit_global_sacl")]
    pub audit_global_sacl: Option<Vec<::models::SmbSettingsGlobalSettingsAuditGlobalSaclItem>>,
    /// Specify the level of logon audit events to log.
    #[serde(rename = "audit_logon")]
    pub audit_logon: Option<String>,
    /// Allow access to .snapshot directories in share subdirectories.
    #[serde(rename = "dot_snap_accessible_child")]
    pub dot_snap_accessible_child: Option<bool>,
    /// Allow access to the .snapshot directory in the root of the share.
    #[serde(rename = "dot_snap_accessible_root")]
    pub dot_snap_accessible_root: Option<bool>,
    /// Show .snapshot directories in share subdirectories.
    #[serde(rename = "dot_snap_visible_child")]
    pub dot_snap_visible_child: Option<bool>,
    /// Show the .snapshot directory in the root of a share.
    #[serde(rename = "dot_snap_visible_root")]
    pub dot_snap_visible_root: Option<bool>,
    /// Indicates whether the server supports signed SMB packets.
    #[serde(rename = "enable_security_signatures")]
    pub enable_security_signatures: Option<bool>,
    /// Specifies the fully-qualified user to use for guest access.
    #[serde(rename = "guest_user")]
    pub guest_user: Option<String>,
    /// Specify whether to ignore EAs on files.
    #[serde(rename = "ignore_eas")]
    pub ignore_eas: Option<bool>,
    /// Specify the number of OneFS driver worker threads per CPU.
    #[serde(rename = "onefs_cpu_multiplier")]
    pub onefs_cpu_multiplier: Option<i32>,
    /// Set the maximum number of OneFS driver worker threads.
    #[serde(rename = "onefs_num_workers")]
    pub onefs_num_workers: Option<i32>,
    /// Indicates whether the server requires signed SMB packets.
    #[serde(rename = "require_security_signatures")]
    pub require_security_signatures: Option<bool>,
    /// Enable Server Side Copy.
    #[serde(rename = "server_side_copy")]
    pub server_side_copy: Option<bool>,
    /// Provides a description of the server.
    #[serde(rename = "server_string")]
    pub server_string: Option<String>,
    /// Specify whether service is enabled.
    #[serde(rename = "service")]
    pub service: Option<bool>,
    /// Specify the number of SRV service worker threads per CPU.
    #[serde(rename = "srv_cpu_multiplier")]
    pub srv_cpu_multiplier: Option<i32>,
    /// Set the maximum number of SRV service worker threads.
    #[serde(rename = "srv_num_workers")]
    pub srv_num_workers: Option<i32>,
    /// Support multichannel.
    #[serde(rename = "support_multichannel")]
    pub support_multichannel: Option<bool>,
    /// Support NetBIOS.
    #[serde(rename = "support_netbios")]
    pub support_netbios: Option<bool>,
    /// Support the SMB2 protocol on the server.
    #[serde(rename = "support_smb2")]
    pub support_smb2: Option<bool>,
}
