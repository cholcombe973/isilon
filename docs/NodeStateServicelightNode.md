# NodeStateServicelightNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | The node service light state (True &#x3D; on). | [default to null]
**error** | **String** | Error message, if the HTTP status returned from this node was not 200. | [optional] [default to null]
**id** | **i32** | Node ID of the node reporting this information. | [optional] [default to null]
**lnn** | **i32** | Logical node number of the node reporting this information. | [optional] [default to null]
**present** | **bool** | This node has a service light. | [optional] [default to null]
**status** | **i32** | Status of the HTTP response from this node if not 200.  If 200, this field does not appear. | [optional] [default to null]
**supported** | **bool** | This node supports a service light. | [optional] [default to null]
**valid** | **bool** | The node service light state is valid (False &#x3D; Error). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


