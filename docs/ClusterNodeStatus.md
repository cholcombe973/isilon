# ClusterNodeStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batterystatus** | [***::models::NodeStatusNodeBatterystatus**](NodeStatusNodeBatterystatus.md) | Battery status information. | [optional] [default to null]
**capacity** | [**Vec<::models::NodeStatusNodeCapacityItem>**](NodeStatusNodeCapacityItem.md) | Storage capacity of this node. | [optional] [default to null]
**cpu** | [***::models::NodeStatusNodeCpu**](NodeStatusNodeCpu.md) | CPU status information for this node. | [optional] [default to null]
**nvram** | [***::models::NodeStatusNodeNvram**](NodeStatusNodeNvram.md) | Node NVRAM information. | [optional] [default to null]
**powersupplies** | [***::models::NodeStatusNodePowersupplies**](NodeStatusNodePowersupplies.md) | Information about this node&#39;s power supplies. | [optional] [default to null]
**release** | **String** | OneFS release. | [optional] [default to null]
**uptime** | **i32** | Seconds this node has been online. | [optional] [default to null]
**version** | **String** | OneFS version. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


