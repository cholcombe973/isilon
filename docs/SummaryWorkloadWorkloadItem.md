# SummaryWorkloadWorkloadItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu** | **f32** | The number (across all cores) of micro-seconds per second. | [default to null]
**job_type** | **String** | The canonical name for the job followed by phase in brackets, ie. &#39;AVscan[1]&#39;, etc... | [optional] [default to null]
**l2** | **f32** | L2 cache hits per second. | [default to null]
**l3** | **f32** | L3 cache hits per second. | [default to null]
**node** | **f32** | The node on which the operation was performed. | [default to null]
**reads** | **f32** | Disk read operations per second. | [default to null]
**system_name** | **String** | The process name, job ID, etc... | [optional] [default to null]
**writes** | **f32** | Disk write operations per second. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


