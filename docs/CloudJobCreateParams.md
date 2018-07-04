# CloudJobCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | **Vec<String>** | The names of accounts for COI restore | [optional] [default to null]
**directories** | **Vec<String>** | Directories addressed by this job | [optional] [default to null]
**expiration_date** | **i32** | The new expiration date in seconds | [optional] [default to null]
**file_matching_pattern** | [***::models::Empty**](Empty.md) | The file filtering logic to find files for this job. (Only applicable for &#39;recall&#39; jobs) | [optional] [default to null]
**files** | **Vec<String>** | Filenames addressed by this job | [optional] [default to null]
**policy** | **String** | The name of an existing cloudpool policy to apply to this job. (Only applicable for &#39;archive&#39; jobs) | [optional] [default to null]
**_type** | **String** | The type of cloud action to be performed by this job. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


