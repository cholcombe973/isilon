# StoragepoolNodepoolExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_enable_l3** | **bool** | Indicates if enabling L3 is possible. L3 cannot be enabled if there are unprovisioned drives. | [default to null]
**health_flags** | **Vec<String>** | An array of containing any health issues with this pool.  If the pool is healthy, the list is empty. | [optional] [default to null]
**id** | **i32** | The system ID given to the node pool. | [default to null]
**l3** | **bool** | Use SSDs in this node pool for L3 cache. | [default to null]
**l3_status** | **String** | &#39;storage&#39; if the &#39;l3&#39; option is disabled. If the l3 option is enabled, &#39;migrating&#39; if any SSDs in this node pool have not yet been migrated to L3. If all SSDs have been migrated, &#39;l3&#39;. | [default to null]
**lnns** | **Vec<i32>** | The nodes that are part of this node pool. | [default to null]
**manual** | **bool** | Whether or not the node pool is manually managed. | [default to null]
**name** | **String** | The node pool name. | [default to null]
**protection_policy** | **String** | The underlying protection policy. | [optional] [default to null]
**tier** | **String** | The name (if named) or system ID of the node pool&#39;s tier, if it is in a tier. Otherwise null. | [optional] [default to null]
**usage** | [***::models::StoragepoolTierUsage**](StoragepoolTierUsage.md) | Total pool usage. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


