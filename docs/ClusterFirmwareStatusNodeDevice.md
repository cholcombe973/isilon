# ClusterFirmwareStatusNodeDevice

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | **String** | The name of the device. | [optional] [default to null]
**mismatch** | **bool** | Is the firmware up-to-date for this component. | [optional] [default to null]
**target_version** | **String** | The target firmware version. | [optional] [default to null]
**_type** | **String** | The device type. | [optional] [default to null]
**upgrade_status** | **String** | The current state of the firmware upgrade for this component. One of the following values: &#39;queued&#39;, &#39;upgrading&#39;, &#39;upgraded&#39;, &#39;error&#39;. or &#39;null&#39;.&#39;null&#39; indicates that the upgrade status is unknown. | [optional] [default to null]
**version** | **String** | The current firmware version. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


