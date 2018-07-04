# SnapshotScheduleExtendedExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alias** | **String** | Alias name to create for each snapshot. | [optional] [default to null]
**duration** | **i32** | Time in seconds added to creation time to construction expiration time. | [optional] [default to null]
**id** | **i32** | The system ID given to the schedule. | [optional] [default to null]
**name** | **String** | The schedule name. | [optional] [default to null]
**next_run** | **i32** | Unix Epoch time of next snapshot to be created. | [optional] [default to null]
**next_snapshot** | **String** | Formatted name (see pattern) of next snapshot to be created. | [optional] [default to null]
**path** | **String** | The /ifs path snapshotted. | [optional] [default to null]
**pattern** | **String** | Pattern expanded with strftime to create snapshot names. | [optional] [default to null]
**schedule** | **String** | The isidate compatible natural language description of the schedule. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


