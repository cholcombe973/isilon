# EventAlertConditionCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | **Vec<String>** | Event Group categories to be alerted: all, 100000000 (SYS_DISK_EVENTS), 200000000 (NODE_STATUS_EVENTS), 300000000 (REBOOT_EVENTS), 400000000 (SW_EVENTS), 500000000 (QUOTA_EVENTS), 600000000 (SNAP_EVENTS), 700000000 (WINNET_EVENTS), 800000000 (FILESYS_EVENTS), 900000000 (HW_EVENTS), 1100000000 (CPOOL_EVENTS) | [optional] [default to null]
**channels** | **Vec<String>** | Channels for alert | [default to null]
**condition** | **String** | Trigger condition for alert | [default to null]
**eventgroup_ids** | **Vec<String>** | Event Group IDs to be alerted | [optional] [default to null]
**id** | **String** | Unique identifier. | [optional] [default to null]
**interval** | **i32** | Required with ONGOING condition only, period in seconds between alerts of ongoing conditions | [optional] [default to null]
**limit** | **i32** | Required with NEW EVENTS condition only, limits the number of alerts sent as events are added | [optional] [default to null]
**name** | **String** | Unique identifier. | [default to null]
**severities** | **Vec<String>** | Severities to be alerted | [optional] [default to null]
**transient** | **i32** | Any eventgroup lasting less than this many seconds is deemed transient and will not generate alerts under this condition. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


