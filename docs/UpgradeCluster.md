# UpgradeCluster

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_overview** | [***::models::UpgradeClusterClusterOverview**](UpgradeClusterClusterOverview.md) | The cluster overview of an upgrade process. | [optional] [default to null]
**cluster_state** | **String** | The different states of an upgrade, rollback, or assessment. One of the following values: &#39;committed&#39;, &#39;upgraded&#39;, &#39;partially upgraded&#39;, &#39;upgrading&#39;, &#39;rolling back&#39;, &#39;assessing&#39;, &#39;error&#39; | [optional] [default to null]
**current_process** | **String** | The current upgrade activity. | [optional] [default to null]
**finish_time** | **String** | The time when a rollback, assessment or upgrade has finished completely. Use ISO 8601 standard. Null if the cluster_state is not &#39;upgraded&#39;. | [optional] [default to null]
**install_image_path** | **String** | The location (path) of the upgrade image which must be within /ifs. Null if the cluster_state is &#39;committed&#39; or &#39;upgraded.&#39; | [optional] [default to null]
**node_median_time** | **i32** | The median time (seconds) to complete each node so far during this upgrade. Before the first node in an upgrade has completed this key will have an associated null value. | [optional] [default to null]
**onefs_version_current** | [***::models::ClusterNodesOnefsVersion**](ClusterNodesOnefsVersion.md) | The current OneFS version before upgrade. | [optional] [default to null]
**onefs_version_upgrade** | [***::models::ClusterNodesOnefsVersion**](ClusterNodesOnefsVersion.md) | The OneFS version the user is attempting to upgrade to. Null if the cluster_state is &#39;committed&#39; or &#39;assessing.&#39; | [optional] [default to null]
**patch_action** | **String** | The most recent patch action performed. | [optional] [default to null]
**patch_name** | **String** | The patch with the most recent patch action. | [optional] [default to null]
**start_time** | **String** | The time when an upgrade, rollback, or assessment was started. Use ISO 8601 standard. Null if the cluster_state is &#39;committed&#39; or &#39;partially upgraded.&#39; | [optional] [default to null]
**upgrade_is_committed** | **bool** | True if upgrade is committed. | [optional] [default to null]
**upgrade_settings** | [***::models::UpgradeClusterUpgradeSettings**](UpgradeClusterUpgradeSettings.md) | The settings necessary when starting an upgrade. Null if the cluster_state is not &#39;upgrading&#39; or &#39;partially upgraded.&#39; or &#39;error&#39;. | [optional] [default to null]
**upgrade_triggered_time** | **String** | Time at which upgrade was originally requested. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


