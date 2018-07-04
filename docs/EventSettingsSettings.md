# EventSettingsSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**heartbeat_interval** | **String** | Interval between heartbeat events. \&quot;daily\&quot;, \&quot;weekly\&quot;, or \&quot;monthly\&quot;. | [optional] [default to null]
**maintenance** | [***::models::EventSettingsSettingsMaintenance**](EventSettingsSettingsMaintenance.md) | Specifies start and duration of maintenance period during which no alerts will be sent for new eventgroups. | [optional] [default to null]
**retention_days** | **i32** | Retention period in days | [optional] [default to null]
**storage_limit** | **i32** | Storage limit in megabytes per terabyte of available storage | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


