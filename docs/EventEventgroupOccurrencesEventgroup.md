# EventEventgroupOccurrencesEventgroup

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**causes** | [**Vec<Vec<String>>**](array.md) | List of eventgroup IDs that may be causes of this occurrence. | [optional] [default to null]
**channels** | **Vec<String>** | List of channels to which alerts are configured for this eventgroup | [optional] [default to null]
**event_count** | **i32** | Number of events linked to this eventgroup. | [optional] [default to null]
**eventgroup_instance** | **String** | Unique identifier of eventgroup instance. | [optional] [default to null]
**id** | **String** | Same as eventgroup_instance. | [optional] [default to null]
**ignore** | **bool** | True if eventgroup is marked as &#39;ignore&#39;. | [optional] [default to null]
**ignore_time** | **i32** | Time eventgroup was marked as &#39;ignore&#39;. | [optional] [default to null]
**last_event** | **i32** | Time the most recent event was added to this eventgroup. | [optional] [default to null]
**resolve_time** | **i32** | When the eventgroup became resolved. | [optional] [default to null]
**resolved** | **bool** | True if eventgroup is resolved. | [optional] [default to null]
**resolver** | **String** | &#39;USER&#39; if the eventgroup was marked resolved via PAPI &lt;event_instance&gt; if eventgroup was marked resolved as a result of an event. | [optional] [default to null]
**sequence** | **i32** | XXX description needed. | [optional] [default to null]
**severity** | **String** | Event Group severity. | [optional] [default to null]
**specifier** | [***::models::Empty**](Empty.md) | A collection of parameters defined per eventgroup_id. | [optional] [default to null]
**time_noticed** | **i32** | Time of first event linked to this eventgroup. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


