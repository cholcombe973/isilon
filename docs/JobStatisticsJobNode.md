# JobStatisticsJobNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu** | [***::models::JobStatisticsJobNodeCpu**](JobStatisticsJobNodeCpu.md) |  | [default to null]
**io** | [***::models::JobStatisticsJobNodeIo**](JobStatisticsJobNodeIo.md) |  | [default to null]
**memory** | [***::models::JobStatisticsJobNodeMemory**](JobStatisticsJobNodeMemory.md) |  | [default to null]
**node** | **i32** | The devid of the node. | [default to null]
**pid** | **i32** | The process ID of the job on this node. | [default to null]
**total_workers** | **i32** | The number of workers for this job on this node. | [default to null]
**workers** | [**Vec<::models::JobStatisticsJobNodeWorker>**](JobStatisticsJobNodeWorker.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


