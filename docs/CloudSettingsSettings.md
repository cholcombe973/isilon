# CloudSettingsSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_policy_defaults** | [***::models::CloudSettingsSettingsCloudPolicyDefaults**](CloudSettingsSettingsCloudPolicyDefaults.md) | The default filepool policy values for cloudpools. | [optional] [default to null]
**retry_coefficient_archive** | **String** | Coefficients in the quadratic function for determining the rest period between successive archive attempts. | [optional] [default to null]
**retry_coefficient_cache_invalidation** | **String** | Coefficients in the quadratic function for determining the rest period between successive cache invalidation attempts. | [optional] [default to null]
**retry_coefficient_cloud_garbage_collection** | **String** | Coefficients in the quadratic function for determining the rest period between successive cloud garbage collection attempts. | [optional] [default to null]
**retry_coefficient_local_garbage_collection** | **String** | Coefficients in the quadratic function for determining the rest period between successive local garbage collection attempts. | [optional] [default to null]
**retry_coefficient_read_ahead** | **String** | Coefficients in the quadratic function for determining the rest period between successive read ahead attempts. | [optional] [default to null]
**retry_coefficient_recall** | **String** | Coefficients in the quadratic function for determining the rest period between successive recall attempts. | [optional] [default to null]
**retry_coefficient_writeback** | **String** | Coefficients in the quadratic function for determining the rest period between successive writeback attempts. | [optional] [default to null]
**sleep_timeout_archive** | [***::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection**](CloudSettingsSettingsSleepTimeoutCloudGarbageCollection.md) | Amount of time to wait between successive file archive operations. | [optional] [default to null]
**sleep_timeout_cache_invalidation** | [***::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection**](CloudSettingsSettingsSleepTimeoutCloudGarbageCollection.md) | Amount of time to wait between successive file cache_invalidation operations. | [optional] [default to null]
**sleep_timeout_cloud_garbage_collection** | [***::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection**](CloudSettingsSettingsSleepTimeoutCloudGarbageCollection.md) | Amount of time to wait between successive file cloud garbage collection operations. | [optional] [default to null]
**sleep_timeout_local_garbage_collection** | [***::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection**](CloudSettingsSettingsSleepTimeoutCloudGarbageCollection.md) | Amount of time to wait between successive file local garbage collection operations. | [optional] [default to null]
**sleep_timeout_read_ahead** | [***::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection**](CloudSettingsSettingsSleepTimeoutCloudGarbageCollection.md) | Amount of time to wait between successive file read ahead operations. | [optional] [default to null]
**sleep_timeout_recall** | [***::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection**](CloudSettingsSettingsSleepTimeoutCloudGarbageCollection.md) | Amount of time to wait between successive file recall operations. | [optional] [default to null]
**sleep_timeout_writeback** | [***::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection**](CloudSettingsSettingsSleepTimeoutCloudGarbageCollection.md) | Amount of time to wait between successive file writeback operations. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


