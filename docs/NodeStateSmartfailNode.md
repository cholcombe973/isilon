# NodeStateSmartfailNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dead** | **bool** | This node is dead (dead_devs). | [optional] [default to null]
**down** | **bool** | This node is down (down_devs). | [optional] [default to null]
**error** | **String** | Error message, if the HTTP status returned from this node was not 200. | [optional] [default to null]
**id** | **i32** | Node ID of the node reporting this information. | [optional] [default to null]
**in_cluster** | **bool** | This node is in the cluster (all_devs). | [optional] [default to null]
**lnn** | **i32** | Logical node number of the node reporting this information. | [optional] [default to null]
**readonly** | **bool** | This node is readonly (ro_devs). | [optional] [default to null]
**shutdown_readonly** | **bool** | This node is shutdown readonly (down_devs). | [optional] [default to null]
**smartfailed** | **bool** | This node is smartfailed (soft_devs). | [optional] [default to null]
**status** | **i32** | Status of the HTTP response from this node if not 200.  If 200, this field does not appear. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

