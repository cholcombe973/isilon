# UpgradeClusterClusterOverview

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nodes_current** | **i32** | Number of nodes running the current OneFS version. | [optional] [default to null]
**nodes_total** | **i32** | Total number of nodes on the cluster. | [optional] [default to null]
**nodes_transitioning** | **i32** | Number of nodes transitioning between OneFS versions. Null if the cluster_state is &#39;committed&#39; or &#39;assessing.&#39; | [optional] [default to null]
**nodes_upgraded** | **i32** | Number of nodes running the upgraded OneFS version. Null if the cluster_state is &#39;committed&#39; or &#39;assessing.&#39; | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


