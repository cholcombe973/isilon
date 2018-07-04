# NdmpContextsBackup

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_expiration_time** | **i32** | Context expiration time | [optional] [default to null]
**context_id** | **String** | Context ID | [optional] [default to null]
**id** | **String** | Unique display id. | [optional] [default to null]
**path** | **String** | The directory of the backup. This is not applicable to restore contexts. | [optional] [default to null]
**sessions** | [**Vec<::models::NdmpContextsBackupSession>**](NdmpContextsBackupSession.md) |  | [optional] [default to null]
**snapid** | **i32** | Snapshot ID reserved for the context. This is not applicable to restore contexts. | [optional] [default to null]
**start_time** | **i32** | Context creation time | [optional] [default to null]
**status** | **String** | Whether the context is active. | [optional] [default to null]
**total_sessions** | **i32** | The number of sessions in the context | [optional] [default to null]
**_type** | **String** | Type of context; It can be bre, backup, and restore | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


