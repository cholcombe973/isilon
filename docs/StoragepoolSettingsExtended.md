# StoragepoolSettingsExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**automatically_manage_io_optimization** | **String** | Automatically manage IO optimization settings on files. | [optional] [default to null]
**automatically_manage_protection** | **String** | Automatically manage protection settings on files. | [optional] [default to null]
**global_namespace_acceleration_enabled** | **bool** | Optimize namespace operations by storing metadata on SSDs. | [optional] [default to null]
**protect_directories_one_level_higher** | **bool** | Automatically add additional protection level to all directories. | [optional] [default to null]
**spillover_enabled** | **bool** | Spill writes into other pools as needed. | [optional] [default to null]
**spillover_target** | [***::models::StoragepoolSettingsSpilloverTarget**](StoragepoolSettingsSpilloverTarget.md) | Target pool for spilled writes. | [optional] [default to null]
**ssd_l3_cache_default_enabled** | **bool** | The L3 Cache default enabled state. This specifies whether L3 Cache should be enabled on new node pools | [optional] [default to null]
**ssd_qab_mirrors** | **String** | Controls number of mirrors of QAB blocks to place on SSDs. | [optional] [default to null]
**ssd_system_btree_mirrors** | **String** | Controls number of mirrors of system B-tree blocks to place on SSDs. | [optional] [default to null]
**ssd_system_delta_mirrors** | **String** | Controls number of mirrors of system delta blocks to place on SSDs. | [optional] [default to null]
**virtual_hot_spare_deny_writes** | **bool** | Deny writes into reserved virtual hot spare space. | [optional] [default to null]
**virtual_hot_spare_hide_spare** | **bool** | Hide reserved virtual hot spare space from free space counts. | [optional] [default to null]
**virtual_hot_spare_limit_drives** | **i32** | The number of drives to reserve for the virtual hot spare, from 0-4. | [optional] [default to null]
**virtual_hot_spare_limit_percent** | **i32** | The percent space to reserve for the virtual hot spare, from 0-20. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


