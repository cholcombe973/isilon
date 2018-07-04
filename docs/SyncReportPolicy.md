# SyncReportPolicy

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | If &#39;copy&#39;, source files will be copied to the target cluster.  If &#39;sync&#39;, the target directory will be made an image of the source directory:  Files and directories that have been deleted on the source, have been moved within the target directory, or no longer match the selection criteria will be deleted from the target directory. | [optional] [default to null]
**file_matching_pattern** | [***::models::SyncJobPolicyFileMatchingPattern**](SyncJobPolicyFileMatchingPattern.md) | A file matching pattern, organized as an OR&#39;ed set of AND&#39;ed file criteria, for example ((a AND b) OR (x AND y)) used to define a set of files with specific properties.  Policies of type &#39;sync&#39; cannot use &#39;path&#39; or time criteria in their matching patterns, but policies of type &#39;copy&#39; can use all listed criteria. | [optional] [default to null]
**name** | **String** | User-assigned name of this sync policy. | [optional] [default to null]
**source_exclude_directories** | **Vec<String>** | Directories that will be excluded from the sync.  Modifying this field will result in a full synchronization of all data. | [optional] [default to null]
**source_include_directories** | **Vec<String>** | Directories that will be included in the sync.  Modifying this field will result in a full synchronization of all data. | [optional] [default to null]
**source_root_path** | **String** | The root directory on the source cluster the files will be synced from.  Modifying this field will result in a full synchronization of all data. | [optional] [default to null]
**target_host** | **String** | Hostname or IP address of sync target cluster.  Modifying the target cluster host can result in the policy being unrunnable if the new target does not match the current target association. | [optional] [default to null]
**target_path** | **String** | Absolute filesystem path on the target cluster for the sync destination. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


