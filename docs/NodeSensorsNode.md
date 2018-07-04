# NodeSensorsNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | **String** | Error message, if the HTTP status returned from this node was not 200. | [optional] [default to null]
**id** | **i32** | Node ID of the node reporting this information. | [optional] [default to null]
**lnn** | **i32** | Logical node number of the node reporting this information. | [optional] [default to null]
**sensors** | [**Vec<::models::NodeSensorsNodeSensor>**](NodeSensorsNodeSensor.md) | This node&#39;s sensor information. | [optional] [default to null]
**status** | **i32** | Status of the HTTP response from this node if not 200.  If 200, this field does not appear. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


