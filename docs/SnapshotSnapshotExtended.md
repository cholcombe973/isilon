# SnapshotSnapshotExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alias** | **String** | Alias name to create for this snapshot. If null, remove any alias. | [optional] [default to null]
**expires** | **i32** | The Unix Epoch time the snapshot will expire and be eligible for automatic deletion. | [optional] [default to null]
**name** | **String** | The user or system supplied snapshot name. This will be null for snapshots pending delete. | [optional] [default to null]
**created** | **i32** | The Unix Epoch time the snapshot was created. | [default to null]
**has_locks** | **bool** | True if the snapshot has one or more locks present see, see the locks subresource of a snapshot for a list of locks. | [default to null]
**id** | **i32** | The system ID given to the snapshot. This is useful for tracking the status of delete pending snapshots. | [default to null]
**path** | **String** | The /ifs path snapshotted. | [optional] [default to null]
**pct_filesystem** | **f32** | Percentage of /ifs used for storing this snapshot. | [default to null]
**pct_reserve** | **f32** | Percentage of configured snapshot reserved used for storing this snapshot. | [default to null]
**schedule** | **String** | The name of the schedule used to create this snapshot, if applicable. | [optional] [default to null]
**shadow_bytes** | **i32** | The amount of shadow bytes referred to by this snapshot. | [default to null]
**size** | **i32** | The amount of storage in bytes used to store this snapshot. | [default to null]
**state** | **String** | Snapshot state. | [default to null]
**target_id** | **i32** | The ID of the snapshot pointed to if this is an alias. | [optional] [default to null]
**target_name** | **String** | The name of the snapshot pointed to if this is an alias. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


