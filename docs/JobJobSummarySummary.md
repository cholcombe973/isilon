# JobJobSummarySummary

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_is_degraded** | **bool** | Whether the cluster is in a degraded state.  Note this is from the perspective of the node handling the query, which might be different from another node. | [default to null]
**connected** | **bool** | Whether isi_job_d instances on all up nodes in the cluster are connected to the coordinator. | [default to null]
**coordinator** | **i32** | The devid of the job engine coordinator. | [default to null]
**disconnected_nodes** | **Vec<i32>** | If connected&#x3D;false, this is the set of devids that are not connected to the coordinator. | [optional] [default to null]
**down_or_read_only_nodes** | **bool** | Whether the cluster has any down or read-only nodes.  These nodes are not considered in the connected property. | [default to null]
**next_jid** | **i32** | The job ID to be assigned to the next job. | [default to null]
**run_degraded** | **bool** | Whether the job engine would allow most jobs to run even when the cluster is in a degraded state. | [default to null]
**stats_ready** | **bool** | Whether the coordinator has recently gathered statistics for all nodes in the cluster. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


