# StoragepoolSettingsSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**automatically_manage_io_optimization** | **String** | Automatically manage IO optimization settings on files. | [default to null]
**automatically_manage_protection** | **String** | Automatically manage protection settings on files. | [default to null]
**global_namespace_acceleration_enabled** | **bool** | Optimize namespace operations by storing metadata on SSDs. | [default to null]
**global_namespace_acceleration_state** | **String** | Whether or not namespace operation optimizations are currently in effect. | [default to null]
**protect_directories_one_level_higher** | **bool** | Automatically add additional protection level to all directories. | [default to null]
**spillover_enabled** | **bool** | Spill writes into other pools as needed. | [default to null]
**spillover_target** | [***::models::StoragepoolSettingsSettingsSpilloverTarget**](StoragepoolSettingsSettingsSpilloverTarget.md) | Target pool for spilled writes. | [default to null]
**ssd_l3_cache_default_enabled** | **bool** | The L3 Cache default enabled state. This specifies whether L3 Cache should be enabled on new node pools. | [default to null]
**ssd_qab_mirrors** | **String** | Controls number of mirrors of QAB blocks to place on SSDs. | [default to null]
**ssd_system_btree_mirrors** | **String** | Controls number of mirrors of system B-tree blocks to place on SSDs. | [default to null]
**ssd_system_delta_mirrors** | **String** | Controls number of mirrors of system delta blocks to place on SSDs. | [default to null]
**virtual_hot_spare_deny_writes** | **bool** | Deny writes into reserved virtual hot spare space. | [default to null]
**virtual_hot_spare_hide_spare** | **bool** | Hide reserved virtual hot spare space from free space counts. | [default to null]
**virtual_hot_spare_limit_drives** | **i32** | The number of drives to reserve for the virtual hot spare, from 0-4. | [default to null]
**virtual_hot_spare_limit_percent** | **i32** | The percent space to reserve for the virtual hot spare, from 0-20. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


