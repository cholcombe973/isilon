# SmbSettingsGlobalExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_based_share_enum** | **bool** | Only enumerate files and folders the requesting user has access to. | [optional] [default to null]
**audit_fileshare** | **String** | Specify level of file share audit events to log. | [optional] [default to null]
**audit_global_sacl** | [**Vec <crate::models::SmbSettingsGlobalSettingsAuditGlobalSaclItem>**](SmbSettingsGlobalSettingsAuditGlobalSaclItem.md) | Specifies a list of permissions to audit. | [optional] [default to null]
**audit_logon** | **String** | Specify the level of logon audit events to log. | [optional] [default to null]
**dot_snap_accessible_child** | **bool** | Allow access to .snapshot directories in share subdirectories. | [optional] [default to null]
**dot_snap_accessible_root** | **bool** | Allow access to the .snapshot directory in the root of the share. | [optional] [default to null]
**dot_snap_visible_child** | **bool** | Show .snapshot directories in share subdirectories. | [optional] [default to null]
**dot_snap_visible_root** | **bool** | Show the .snapshot directory in the root of a share. | [optional] [default to null]
**enable_security_signatures** | **bool** | Indicates whether the server supports signed SMB packets. | [optional] [default to null]
**guest_user** | **String** | Specifies the fully-qualified user to use for guest access. | [optional] [default to null]
**ignore_eas** | **bool** | Specify whether to ignore EAs on files. | [optional] [default to null]
**onefs_cpu_multiplier** | **i32** | Specify the number of OneFS driver worker threads per CPU. | [optional] [default to null]
**onefs_num_workers** | **i32** | Set the maximum number of OneFS driver worker threads. | [optional] [default to null]
**require_security_signatures** | **bool** | Indicates whether the server requires signed SMB packets. | [optional] [default to null]
**server_side_copy** | **bool** | Enable Server Side Copy. | [optional] [default to null]
**server_string** | **String** | Provides a description of the server. | [optional] [default to null]
**service** | **bool** | Specify whether service is enabled. | [optional] [default to null]
**srv_cpu_multiplier** | **i32** | Specify the number of SRV service worker threads per CPU. | [optional] [default to null]
**srv_num_workers** | **i32** | Set the maximum number of SRV service worker threads. | [optional] [default to null]
**support_multichannel** | **bool** | Support multichannel. | [optional] [default to null]
**support_netbios** | **bool** | Support NetBIOS. | [optional] [default to null]
**support_smb2** | **bool** | Support the SMB2 protocol on the server. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


