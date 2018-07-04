# SmbShareCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_based_enumeration** | **bool** | Only enumerate files and folders the requesting user has access to. | [optional] [default to null]
**access_based_enumeration_root_only** | **bool** | Access-based enumeration on only the root directory of the share. | [optional] [default to null]
**allow_delete_readonly** | **bool** | Allow deletion of read-only files in the share. | [optional] [default to null]
**allow_execute_always** | **bool** | Allows users to execute files they have read rights for. | [optional] [default to null]
**allow_variable_expansion** | **bool** | Allow automatic expansion of variables for home directories. | [optional] [default to null]
**auto_create_directory** | **bool** | Automatically create home directories. | [optional] [default to null]
**browsable** | **bool** | Share is visible in net view and the browse list. | [optional] [default to null]
**ca_timeout** | **i32** | Persistent open timeout for the share. | [optional] [default to null]
**ca_write_integrity** | **String** | Specify the level of write-integrity on continuously available shares. | [optional] [default to null]
**change_notify** | **String** | Level of change notification alerts on the share. | [optional] [default to null]
**continuously_available** | **bool** | Specify if persistent opens are allowed on the share. | [optional] [default to null]
**create_path** | **bool** | Create path if does not exist. | [optional] [default to null]
**create_permissions** | **String** | Create permissions for new files and directories in share. | [optional] [default to null]
**csc_policy** | **String** | Client-side caching policy for the shares. | [optional] [default to null]
**description** | **String** | Description for this SMB share. | [optional] [default to null]
**directory_create_mask** | **i32** | Directory create mask bits. | [optional] [default to null]
**directory_create_mode** | **i32** | Directory create mode bits. | [optional] [default to null]
**file_create_mask** | **i32** | File create mask bits. | [optional] [default to null]
**file_create_mode** | **i32** | File create mode bits. | [optional] [default to null]
**file_filter_extensions** | **Vec<String>** | Specifies the list of file extensions. | [optional] [default to null]
**file_filter_type** | **String** | Specifies if filter list is for deny or allow. Default is deny. | [optional] [default to null]
**file_filtering_enabled** | **bool** | Enables file filtering on this zone. | [optional] [default to null]
**hide_dot_files** | **bool** | Hide files and directories that begin with a period &#39;.&#39;. | [optional] [default to null]
**host_acl** | **Vec<String>** | An ACL expressing which hosts are allowed access. A deny clause must be the final entry. | [optional] [default to null]
**impersonate_guest** | **String** | Specify the condition in which user access is done as the guest account. | [optional] [default to null]
**impersonate_user** | **String** | User account to be used as guest account. | [optional] [default to null]
**inheritable_path_acl** | **bool** | Set the inheritable ACL on the share path. | [optional] [default to null]
**mangle_byte_start** | **i32** | Specifies the wchar_t starting point for automatic byte mangling. | [optional] [default to null]
**mangle_map** | **Vec<String>** | Character mangle map. | [optional] [default to null]
**name** | **String** | Share name. | [default to null]
**ntfs_acl_support** | **bool** | Support NTFS ACLs on files and directories. | [optional] [default to null]
**oplocks** | **bool** | Support oplocks. | [optional] [default to null]
**path** | **String** | Path of share within /ifs. | [default to null]
**permissions** | [**Vec<::models::SmbSharePermission>**](SmbSharePermission.md) | Specifies an ordered list of permission modifications. | [optional] [default to null]
**run_as_root** | [**Vec<::models::AuthAccessAccessItemFileGroup>**](AuthAccessAccessItemFileGroup.md) | Allow account to run as root. | [optional] [default to null]
**strict_ca_lockout** | **bool** | Specifies if persistent opens would do strict lockout on the share. | [optional] [default to null]
**strict_flush** | **bool** | Handle SMB flush operations. | [optional] [default to null]
**strict_locking** | **bool** | Specifies whether byte range locks contend against SMB I/O. | [optional] [default to null]
**zone** | **String** | Name of the access zone to which to move this SMB share | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


