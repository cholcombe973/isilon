# EventEventlistEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**devid** | **i32** | The device id of the node if it is still part of the cluster otherwise None. | [optional] [default to null]
**ended** | **f32** | Time event was ended (eventgroup resolved) | [optional] [default to null]
**event** | **i32** | Integer identifier of the event type | [optional] [default to null]
**id** | **String** | Unique identifier of event occurrence. | [optional] [default to null]
**lnn** | **i32** | The lnn of the node if it is still part of the cluster, otherwise None. | [optional] [default to null]
**message** | **String** | Human readable description | [optional] [default to null]
**resolve_time** | **i32** | Time the event was resolved. | [optional] [default to null]
**severity** | **String** | Severity of event occurrence. | [optional] [default to null]
**specifier** | [***::models::Empty**](Empty.md) | A collection of parameters defined per event. | [optional] [default to null]
**time** | **i32** | Time event was detected as UNIX timestamp. | [optional] [default to null]
**value** | **f32** | Value of measurement associated with this event. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


