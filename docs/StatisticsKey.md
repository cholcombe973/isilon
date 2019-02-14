# StatisticsKey

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation_type** | **String** | Type of aggregation used in down-sampling. | [default to null]
**base_name** | **String** | Name of key this keys is derived from, if any. | [optional] [default to null]
**default_cache_time** | **i32** | Default time in seconds system will used cached values. | [default to null]
**description** | **String** | Description of statistics key. | [default to null]
**key** | **String** | Key name. | [default to null]
**policies** | [**Vec <crate::models::StatisticsKeyPolicy>**](StatisticsKeyPolicy.md) | List of effective history policies for key. | [optional] [default to null]
**policy_cache_time** | **i32** | Configured time in seconds system will used cached values. | [optional] [default to null]
**real_name** | **String** | Name of real key if this is an alias. | [optional] [default to null]
**scope** | **String** | Scope of key. | [default to null]
**_type** | **String** | Data type of key values. | [default to null]
**units** | **String** | Units of key values. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


