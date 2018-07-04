# NodeHardwareNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chassis** | **String** | Name of this node&#39;s chassis. | [optional] [default to null]
**chassis_code** | **String** | Chassis code of this node (1U, 2U, etc.). | [optional] [default to null]
**chassis_count** | **String** | Number of chassis making up this node. | [optional] [default to null]
**chassis_depth** | **String** | Chassis depth for this node if applicable (Normal, Deep, Unknown). If not supported: Unknown. If Logic to determine chassis depth fails: Unknown. If PSI_Get fails: Unknown. PSI_Get can fail if PSI not initialized, or key does not exist. | [optional] [default to null]
**class** | **String** | Class of this node (storage, accelerator, etc.). | [optional] [default to null]
**code_name** | **String** | Code name of this node if applicable (Minnetonka, MiniHuron, Huron, Union, Tahoe, Superior, Unknown). If not supported: Unknown. If Logic to determine code name fails: Unknown. If PSI_Get fails: Unknown. PSI_Get can fail if PSI not initialized, or key does not exist. | [optional] [default to null]
**compute_type** | **String** | Type of compute node if applicable (Low, Medium, High, Turbo, Ultra, Unknown). If not supported: Unknown. If Logic to determine compute type fails: Unknown. If PSI_Get fails: Unknown. PSI_Get can fail if PSI not initialized, or key does not exist. | [optional] [default to null]
**configuration_id** | **String** | Node configuration ID. | [optional] [default to null]
**cpu** | **String** | Manufacturer and model of this node&#39;s CPU. | [optional] [default to null]
**disk_controller** | **String** | Manufacturer and model of this node&#39;s disk controller. | [optional] [default to null]
**disk_expander** | **String** | Manufacturer and model of this node&#39;s disk expander. | [optional] [default to null]
**family_code** | **String** | Family code of this node (X, S, NL, etc.). | [optional] [default to null]
**flash_drive** | **String** | Manufacturer, model, and device id of this node&#39;s flash drive. | [optional] [default to null]
**generation_code** | **String** | Generation code of this node. | [optional] [default to null]
**hwgen** | **String** | Isilon hardware generation name. | [optional] [default to null]
**id** | **i32** | Node ID (Device Number) of this node. | [optional] [default to null]
**imb_version** | **String** | Version of this node&#39;s Isilon Management Board. | [optional] [default to null]
**infiniband** | **String** | Infiniband card type. | [optional] [default to null]
**lcd_version** | **String** | Version of the LCD panel. | [optional] [default to null]
**lnn** | **i32** | Logical Node Number (LNN) of this node. | [optional] [default to null]
**model** | **String** | Isilon node model identifier string (S200, X410, Infinity-H500, etc.). | [optional] [default to null]
**model_code** | **String** | Isilon node model code string (S200, X410, H500, etc.). | [optional] [default to null]
**motherboard** | **String** | Manufacturer and model of this node&#39;s motherboard. | [optional] [default to null]
**net_interfaces** | **String** | Description of all this node&#39;s network interfaces. | [optional] [default to null]
**node_slot_id** | **i32** | Position of node within chassis (e.g., 1-4 for Infinity chassis). -1 for error or not supported. | [optional] [default to null]
**nvram** | **String** | Manufacturer and model of this node&#39;s NVRAM board. | [optional] [default to null]
**peer_serial_number** | **String** | Serial number of this node&#39;s peer/buddy node.(Infinity Only) | [optional] [default to null]
**performance_code** | **String** | Performance code of this node, if applicable (2, 4, 5, etc.). | [optional] [default to null]
**powersupplies** | **Vec<String>** | Description strings for each power supply on this node. | [optional] [default to null]
**processor** | **String** | Number of processors and cores on this node. | [optional] [default to null]
**product** | **String** | Isilon product name. | [optional] [default to null]
**ram** | **i32** | Size of RAM in bytes. | [optional] [default to null]
**serial_number** | **String** | Serial number of this node. | [optional] [default to null]
**series** | **String** | Series of this node (X, I, NL, etc.). | [optional] [default to null]
**sled_drive_count** | **i32** | Size of drive sleds in node, if applicable. Expected values: 3, 4, 6. 0 if unable to determine sled size. -1 for error or not supported. If PSI_Get fails: -1. PSI_Get can fail if PSI not initialized, or key does not exist. | [optional] [default to null]
**storage_class** | **String** | Storage class of this node (storage or diskless). | [optional] [default to null]
**tier** | **i32** | Platform tier level of this node if applicable (1-4 are defined, 0 for unknown or not supported, -1 for error). If not supported: 0. If Logic to determine tier fails: 0 for unknown. If PSI_Get fails: -1 for error. PSI_Get can fail if PSI not initialized, or key does not exist. | [optional] [default to null]
**top_level_assembly_serial_number** | **String** | Serial number of the top level assembly of this node.(Infinity Only) | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


