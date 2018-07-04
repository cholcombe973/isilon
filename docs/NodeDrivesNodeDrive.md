# NodeDrivesNodeDrive

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bay_group** | **String** | The name of the bay group this drive belongs to. | [optional] [default to null]
**baynum** | **i32** | Numerical representation of this drive&#39;s bay. | [optional] [default to null]
**blocks** | **i32** | Number of blocks on this drive. | [optional] [default to null]
**chassis** | **i32** | The chassis number which contains this drive. | [optional] [default to null]
**devname** | **String** | This drive&#39;s device name. | [optional] [default to null]
**firmware** | [***::models::NodeDrivesNodeDriveFirmware**](NodeDrivesNodeDriveFirmware.md) | Drive firmware information. | [optional] [default to null]
**handle** | **i32** | Drive_d&#39;s handle representation for this drive | [optional] [default to null]
**interface_type** | **String** | String representtation of this drive&#39;s interface type. | [optional] [default to null]
**lnum** | **i32** | This drive&#39;s logical drive number in IFS. | [optional] [default to null]
**locnstr** | **String** | String representation of this drive&#39;s physical location. | [optional] [default to null]
**logical_block_length** | **i32** | Size of a logical block on this drive. | [optional] [default to null]
**media_type** | **String** | String representation of this drive&#39;s media type. | [optional] [default to null]
**model** | **String** | This drive&#39;s manufacturer and model. | [optional] [default to null]
**pending_actions** | **Vec<String>** | This drive&#39;s current outstanding actions. For example, \&quot;add\&quot; or \&quot;firmware_update\&quot;. | [optional] [default to null]
**physical_block_length** | **i32** | Size of a physical block on this drive. | [optional] [default to null]
**present** | **bool** | Indicates whether this drive is physically present in the node. | [optional] [default to null]
**purpose** | **String** | This drive&#39;s purpose in the DRV state machine. | [optional] [default to null]
**purpose_description** | **String** | Description of this drive&#39;s purpose. | [optional] [default to null]
**serial** | **String** | Serial number for this drive. | [optional] [default to null]
**ui_state** | **String** | This drive&#39;s state as presented to the UI. | [optional] [default to null]
**wwn** | **String** | The drive&#39;s &#39;worldwide name&#39; from its NAA identifiers. | [optional] [default to null]
**x_loc** | **i32** | This drive&#39;s x-axis grid location. | [optional] [default to null]
**y_loc** | **i32** | This drive&#39;s y-axis grid location. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


