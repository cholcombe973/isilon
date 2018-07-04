# NodeStateReadonlyNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed** | **bool** | The current read-only mode allowed status for the node. | [optional] [default to null]
**enabled** | **bool** | The current read-only user mode status for the node. NOTE: If read-only mode is currently disallowed for this node, it will remain read/write until read-only mode is allowed again. This value only sets or clears any user-specified requests for read-only mode. If the node has been placed into read-only mode by the system, it will remain in read-only mode until the system conditions which triggered read-only mode have cleared. | [optional] [default to null]
**error** | **String** | Error message, if the HTTP status returned from this node was not 200. | [optional] [default to null]
**id** | **i32** | Node ID of the node reporting this information. | [optional] [default to null]
**lnn** | **i32** | Logical node number of the node reporting this information. | [optional] [default to null]
**mode** | **bool** | The current read-only mode status for the node. | [optional] [default to null]
**status** | **String** | The current read-only mode status description for the node. | [optional] [default to null]
**valid** | **bool** | The read-only state values are valid (False &#x3D; Error). | [optional] [default to null]
**value** | **i32** | The current read-only value (enumerated bitfield) for the node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


