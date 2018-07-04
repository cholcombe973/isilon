# SyncSettingsExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**burst_memory_constraint** | **i32** | The per-worker burst socket memory constraint, in bytes. | [optional] [default to null]
**burst_socket_buffer_size** | **i32** | The per-worker burst socket buffer coalesced data, in bytes. | [optional] [default to null]
**force_interface** | **bool** | NOTE: This field should not be changed without the help of Isilon support.  Default for the \&quot;force_interface\&quot; property that will be applied to each new sync policy unless otherwise specified at the time of policy creation.  Determines whether data is sent only through the subnet and pool specified in the \&quot;source_network\&quot; field. This option can be useful if there are multiple interfaces for the given source subnet. | [optional] [default to null]
**report_email** | **Vec<String>** | Email sync reports to these addresses. | [optional] [default to null]
**report_max_age** | **i32** | The default length of time (in seconds) a policy report will be stored. | [optional] [default to null]
**report_max_count** | **i32** | The default maximum number of reports to retain for a policy. | [optional] [default to null]
**restrict_target_network** | **bool** | Default for the \&quot;restrict_target_network\&quot; property that will be applied to each new sync policy unless otherwise specified at the time of policy creation.  If you specify true, and you specify a SmartConnect zone in the \&quot;target_host\&quot; field, replication policies will connect only to nodes in the specified SmartConnect zone.  If you specify false, replication policies are not restricted to specific nodes on the target cluster. | [optional] [default to null]
**rpo_alerts** | **bool** | If disabled, no RPO alerts will be generated. | [optional] [default to null]
**service** | **String** | Specifies if the SyncIQ service currently on, paused, or off.  If paused, all sync jobs will be paused.  If turned off, all jobs will be canceled. | [optional] [default to null]
**source_network** | [***::models::SyncPolicySourceNetwork**](SyncPolicySourceNetwork.md) | Restricts replication policies on the local cluster to running on the specified subnet and pool. | [optional] [default to null]
**tw_chkpt_interval** | **i32** | The interval (in seconds) in which treewalk syncs are forced to checkpoint. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


