# AntivirusPolicy

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | A description for the policy. | [optional] [default to null]
**enabled** | **bool** | Whether the policy is enabled. | [optional] [default to null]
**force_run** | **bool** | Forces the scan to run regardless of whether the files were recently scanned. | [optional] [default to null]
**impact** | **String** |  | [optional] [default to null]
**name** | **String** | The name of the policy. | [optional] [default to null]
**paths** | **Vec<String>** | Paths to include in the scan. | [optional] [default to null]
**recursion_depth** | **i32** | The depth to recurse in directories.  The default of -1 gives unlimited recursion. | [optional] [default to null]
**schedule** | **String** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


