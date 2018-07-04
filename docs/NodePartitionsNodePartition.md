# NodePartitionsNodePartition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_size** | **i32** | The block size used for the reported partition information. | [optional] [default to null]
**capacity** | **i32** | Total blocks on this file system partition. | [optional] [default to null]
**component_devices** | **String** | Comma separated list of devices used for this file system partition. | [optional] [default to null]
**mount_point** | **String** | Directory on which this partition is mounted. | [optional] [default to null]
**percent_used** | **String** | Used blocks on this file system partition, expressed as a percentage. | [optional] [default to null]
**statfs** | [***::models::NodePartitionsNodePartitionStatfs**](NodePartitionsNodePartitionStatfs.md) | System partition details as provided by statfs(2). | [optional] [default to null]
**used** | **i32** | Used blocks on this file system partition. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


