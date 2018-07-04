# TargetPolicy

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failover_failback_state** | **String** | The condition of this policy with respect to sync failover/failback. | [default to null]
**id** | **String** | The system ID given to this sync policy. | [default to null]
**last_job_state** | **String** | The state of the last job run for this policy. | [default to null]
**last_source_coordinator_ip** | **String** | The IP address from which a SyncIQ coordinator daemon most recently connected to this cluster to update it about the progress of a job for this policy. | [default to null]
**last_update_from_source** | **i32** | The last time this cluster was updated with sync information from the source cluster for this policy, in unix epoch seconds.  Null if no such update has occurred yet. | [optional] [default to null]
**legacy_policy** | **bool** | Was this policy defined by a OneFS version earlier than 6.0? (Pre-6.0 policies did not have the target policy concept and canceling from the target side will not be available.) | [default to null]
**name** | **String** | User-assigned name of this sync policy. | [default to null]
**source_cluster_guid** | **String** | Unique identifier for the source cluster. | [default to null]
**source_host** | **String** | Hostname or IP address of sync source cluster. | [default to null]
**target_path** | **String** | Absolute filesystem path on the target cluster for the sync destination. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


