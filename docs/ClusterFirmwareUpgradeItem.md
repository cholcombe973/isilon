# ClusterFirmwareUpgradeItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exclude_device** | **String** | Exclude the specified devices in the firmware upgrade. | [optional] [default to null]
**exclude_type** | **String** | Exclude the specified device type in the firmware upgrade. | [optional] [default to null]
**include_device** | **String** | Include the specified devices in the firmware upgrade. | [optional] [default to null]
**include_type** | **String** | Include the specified device type in the firmware upgrade. | [optional] [default to null]
**no_burn** | **bool** | Do not burn the firmware. | [optional] [default to null]
**no_reboot** | **bool** | Do not reboot the node after an upgrade | [optional] [default to null]
**no_verify** | **bool** | Do not verify the firmware upgrade after an upgrade. | [optional] [default to null]
**nodes_to_upgrade** | **Vec<i32>** | The nodes scheduled for upgrade. Order in array determines queue position number. &#39;All&#39; and null option will upgrade all nodes in &lt;lnn&gt; order. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


