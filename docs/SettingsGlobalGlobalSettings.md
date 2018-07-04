# SettingsGlobalGlobalSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alloc_retries** | **i32** | Specifies the number of times to retry an ID allocation before failing. | [optional] [default to null]
**gid_range_enabled** | **bool** | If true, allocates GIDs from a fixed range. | [optional] [default to null]
**gid_range_max** | **i32** | Specifies the ending number for a fixed range from which GIDs are allocated. | [optional] [default to null]
**gid_range_min** | **i32** | Specifies the starting number for a fixed range from which GIDs are allocated. | [optional] [default to null]
**gid_range_next** | **i32** | Specifies the next GID to allocate. | [optional] [default to null]
**group_uid** | **i32** | Specifies the user iD for a group when requested by the kernel. | [optional] [default to null]
**load_providers** | **Vec<String>** | Specifies which providers are loaded by the authentication daemon (lsassd). | [optional] [default to null]
**min_mapped_rid** | **i32** | Starts the RID in the local domain to map a UID and a GID. | [optional] [default to null]
**null_gid** | **i32** | Specifies an alternative GID when the kernel is unable to retrieve a GID for a persona. | [optional] [default to null]
**null_uid** | **i32** | Specifies an alternative UID when the kernel is unable to retrieve a UID for a persona. | [optional] [default to null]
**on_disk_identity** | **String** | Specifies the type of identity that is stored on disk. | [optional] [default to null]
**rpc_block_time** | **i32** | Specifies the minimum amount of time in milliseconds to wait before performing an oprestart. | [optional] [default to null]
**rpc_max_requests** | **i32** | Specifies the maximum number of outstanding RPC requests. | [optional] [default to null]
**rpc_timeout** | **i32** | Specifies the maximum amount of time in seconds to wait for an idmap response. | [optional] [default to null]
**send_ntlmv2** | **bool** | If true, sends NTLMv2 responses. | [optional] [default to null]
**space_replacement** | **String** | Specifies the space replacement character. | [optional] [default to null]
**system_gid_threshold** | **i32** | Specifies the minimum GID to attempt to look up in the idmap database. | [optional] [default to null]
**system_uid_threshold** | **i32** | Specifies the minimum UID to attempt to look up in the idmap database. | [optional] [default to null]
**uid_range_enabled** | **bool** | If true, allocates UIDs from a fixed range. | [optional] [default to null]
**uid_range_max** | **i32** | Specifies the ending number for a fixed range from which UIDs are allocated. | [optional] [default to null]
**uid_range_min** | **i32** | Specifies the starting number for a fixed range from which UIDs are allocated. | [optional] [default to null]
**uid_range_next** | **i32** | Specifies the next UID to allocate. | [optional] [default to null]
**unknown_gid** | **i32** | Specifies the GID for the unknown (anonymous) group. | [optional] [default to null]
**unknown_uid** | **i32** | Specifies the UID for the unknown (anonymous) user. | [optional] [default to null]
**user_object_cache_size** | **i32** | Specifies the maximum size (in bytes) of the security object cache in the authentication daemon. | [optional] [default to null]
**workgroup** | **String** | Specifies the NetBIOS workgroup or domain. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


