# ClusterFirmwareStatusNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**devices** | [**Vec <crate::models::ClusterFirmwareStatusNodeDevice>**](ClusterFirmwareStatusNodeDevice.md) | List of the firmware status for hardware components on the node. | [optional] [default to null]
**lnn** | **i32** | The lnn of the node. | [optional] [default to null]
**node_unavailable** | **bool** | Node is unavailable. | [optional] [default to null]
**package** | [**Vec <crate::models::ClusterFirmwareStatusNodePackageItem>**](ClusterFirmwareStatusNodePackageItem.md) | List of the firmware binary information for the installed firmware package. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


