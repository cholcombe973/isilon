# ClusterNodeExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**drive_d_config** | [***::models::ClusterNodeDriveDConfig**](ClusterNodeDriveDConfig.md) | An object containing a node&#39;s drive subsystem XML configuration file. | [optional] [default to null]
**drives** | [**Vec<::models::NodeDrivesNodeDrive>**](NodeDrivesNodeDrive.md) | List of the drives in this node. | [optional] [default to null]
**hardware** | [***::models::ClusterNodeHardware**](ClusterNodeHardware.md) | Node hardware identifying information (static). | [optional] [default to null]
**id** | **i32** | Node ID (Device Number) of this node. | [optional] [default to null]
**lnn** | **i32** | Logical Node Number (LNN) of this node. | [optional] [default to null]
**partitions** | [***::models::ClusterNodePartitions**](ClusterNodePartitions.md) | Node partition information. | [optional] [default to null]
**sensors** | [***::models::ClusterNodeSensors**](ClusterNodeSensors.md) | Node sensor information (hardware reported). | [optional] [default to null]
**sleds** | [**Vec<::models::NodeSledsNodeSled>**](NodeSledsNodeSled.md) | List of the sleds in this node. | [optional] [default to null]
**state** | [***::models::ClusterNodeStateExtended**](ClusterNodeStateExtended.md) | Node state information (reported and modifiable). | [optional] [default to null]
**status** | [***::models::ClusterNodeStatus**](ClusterNodeStatus.md) | Node status information (hardware reported). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


