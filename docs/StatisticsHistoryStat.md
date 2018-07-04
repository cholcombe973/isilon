# StatisticsHistoryStat

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**devid** | **i32** | Devid of node of statistic or 0 for cluster scoped statistics. | [default to null]
**error** | **String** | Key specific error string, if applicable. | [optional] [default to null]
**error_code** | **i32** | Key specific error number, if applicable. | [optional] [default to null]
**key** | **String** | Key name of statistic. | [default to null]
**resolution** | **i32** | The interval for which these results were figured (averaged against.) | [default to null]
**values** | [**Vec<::models::StatisticsHistoryStatValue>**](StatisticsHistoryStatValue.md) | Time-series values. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


