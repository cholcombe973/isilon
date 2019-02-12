# NodeStatusNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batterystatus** | [***::models::NodeStatusNodeBatterystatus**](NodeStatusNodeBatterystatus.md) | Battery status information. | [optional] [default to null]
**capacity** | [**Vec <crate::models::NodeStatusNodeCapacityItem>**](NodeStatusNodeCapacityItem.md) | Storage capacity of this node. | [optional] [default to null]
**cpu** | [***::models::NodeStatusNodeCpu**](NodeStatusNodeCpu.md) | CPU status information for this node. | [optional] [default to null]
**error** | **String** | Error message, if the HTTP status returned from this node was not 200. | [optional] [default to null]
**id** | **i32** | Node ID of the node reporting this information. | [optional] [default to null]
**lnn** | **i32** | Logical node number of the node reporting this information. | [optional] [default to null]
**nvram** | [***::models::NodeStatusNodeNvram**](NodeStatusNodeNvram.md) | Node NVRAM information. | [optional] [default to null]
**powersupplies** | [***::models::NodeStatusNodePowersupplies**](NodeStatusNodePowersupplies.md) | Information about this node&#39;s power supplies. | [optional] [default to null]
**release** | **String** | OneFS release. | [optional] [default to null]
**status** | **i32** | Status of the HTTP response from this node if not 200.  If 200, this field does not appear. | [optional] [default to null]
**uptime** | **i32** | Seconds this node has been online. | [optional] [default to null]
**version** | **String** | OneFS version. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


