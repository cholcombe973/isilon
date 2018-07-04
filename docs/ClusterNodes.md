# ClusterNodes

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | [***::models::ClusterNodesError**](ClusterNodesError.md) | The current OneFS version before upgrade. | [optional] [default to null]
**last_action** | **String** | The last action performed to completion/failure on this node.  Null if the node_state is &#39;committed&#39; or &#39;assessing.&#39; One of the following values: &#39;upgrade&#39;, &#39;rollback&#39;. | [optional] [default to null]
**last_action_result** | **String** | Did the node pass upgrade or rollback without failing? Null if the node_state is &#39;committed.&#39; One of the following values: &#39;pass&#39;, &#39;fail&#39;, null | [optional] [default to null]
**lnn** | **i32** | The lnn of the node. | [optional] [default to null]
**node_state** | **String** | \\e The state of the node during the upgrade, rollback, or assessment. One of the following values: &#39;committed&#39;, &#39;upgraded&#39;, &#39;upgrading&#39;, &#39;rolling back&#39;, &#39;assessing&#39;, &#39;error&#39; | [optional] [default to null]
**onefs_version** | [***::models::ClusterNodesOnefsVersion**](ClusterNodesOnefsVersion.md) | The current OneFS version before upgrade. | [optional] [default to null]
**progress** | **i32** | What step is the upgrade, assessment, or rollback in? To show via progress indicator. NOTE: the value is an integer between 0 and 100 (percent) | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


