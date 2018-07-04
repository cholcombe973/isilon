# CloudJobFiles

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_matching_pattern** | [***::models::Empty**](Empty.md) | The file filtering logic to find files for this job | [optional] [default to null]
**names** | [**Vec<::models::CloudJobFilesName>**](CloudJobFilesName.md) | A list of files to be addressed by this job.  (Note* these are only reported when audit_level is &#39;high&#39; | [optional] [default to null]
**total** | **i32** | The total number of files addressed by this job | [optional] [default to null]
**total_canceled** | **i32** | The number of canceled files | [optional] [default to null]
**total_failed** | **i32** | The number of files which failed | [optional] [default to null]
**total_pending** | **i32** | The number of files pending action | [optional] [default to null]
**total_processing** | **i32** | The number of files currently being processed | [optional] [default to null]
**total_succeeded** | **i32** | The total number of files successfully completed | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


