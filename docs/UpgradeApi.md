# \UpgradeApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster_add_remaining_node**](UpgradeApi.md#create_cluster_add_remaining_node) | **Post** /platform/3/upgrade/cluster/add_remaining_nodes | 
[**create_cluster_archive_item**](UpgradeApi.md#create_cluster_archive_item) | **Post** /platform/3/upgrade/cluster/archive | 
[**create_cluster_assess_item**](UpgradeApi.md#create_cluster_assess_item) | **Post** /platform/5/upgrade/cluster/assess | 
[**create_cluster_commit_item**](UpgradeApi.md#create_cluster_commit_item) | **Post** /platform/3/upgrade/cluster/commit | 
[**create_cluster_firmware_assess_item**](UpgradeApi.md#create_cluster_firmware_assess_item) | **Post** /platform/3/upgrade/cluster/firmware/assess | 
[**create_cluster_firmware_upgrade_item**](UpgradeApi.md#create_cluster_firmware_upgrade_item) | **Post** /platform/3/upgrade/cluster/firmware/upgrade | 
[**create_cluster_patch_abort_item**](UpgradeApi.md#create_cluster_patch_abort_item) | **Post** /platform/3/upgrade/cluster/patch/abort | 
[**create_cluster_patch_patch**](UpgradeApi.md#create_cluster_patch_patch) | **Post** /platform/3/upgrade/cluster/patch/patches | 
[**create_cluster_retry_last_action_item**](UpgradeApi.md#create_cluster_retry_last_action_item) | **Post** /platform/3/upgrade/cluster/retry_last_action | 
[**create_cluster_rollback_item**](UpgradeApi.md#create_cluster_rollback_item) | **Post** /platform/3/upgrade/cluster/rollback | 
[**create_cluster_upgrade_item**](UpgradeApi.md#create_cluster_upgrade_item) | **Post** /platform/5/upgrade/cluster/upgrade | 
[**create_hardware_start_item**](UpgradeApi.md#create_hardware_start_item) | **Post** /platform/5/upgrade/hardware/start | 
[**create_hardware_stop_item**](UpgradeApi.md#create_hardware_stop_item) | **Post** /platform/5/upgrade/hardware/stop | 
[**delete_cluster_patch_patch**](UpgradeApi.md#delete_cluster_patch_patch) | **Delete** /platform/3/upgrade/cluster/patch/patches/{ClusterPatchPatchId} | 
[**get_cluster_firmware_progress**](UpgradeApi.md#get_cluster_firmware_progress) | **Get** /platform/3/upgrade/cluster/firmware/progress | 
[**get_cluster_firmware_status**](UpgradeApi.md#get_cluster_firmware_status) | **Get** /platform/3/upgrade/cluster/firmware/status | 
[**get_cluster_node**](UpgradeApi.md#get_cluster_node) | **Get** /platform/3/upgrade/cluster/nodes/{ClusterNodeId} | 
[**get_cluster_nodes**](UpgradeApi.md#get_cluster_nodes) | **Get** /platform/3/upgrade/cluster/nodes | 
[**get_cluster_patch_patch**](UpgradeApi.md#get_cluster_patch_patch) | **Get** /platform/3/upgrade/cluster/patch/patches/{ClusterPatchPatchId} | 
[**get_hardware_status**](UpgradeApi.md#get_hardware_status) | **Get** /platform/5/upgrade/hardware/status | 
[**get_upgrade_cluster**](UpgradeApi.md#get_upgrade_cluster) | **Get** /platform/4/upgrade/cluster | 
[**list_cluster_patch_patches**](UpgradeApi.md#list_cluster_patch_patches) | **Get** /platform/3/upgrade/cluster/patch/patches | 
[**update_cluster_upgrade**](UpgradeApi.md#update_cluster_upgrade) | **Put** /platform/5/upgrade/cluster/upgrade | 


# **create_cluster_add_remaining_node**
>crate::models::Empty create_cluster_add_remaining_node(ctx, cluster_add_remaining_node)


Let system absorb any remaining or new nodes inside the existing upgrade.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_add_remaining_node** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_archive_item**
>crate::models::Empty create_cluster_archive_item(ctx, cluster_archive_item)


Start an archive of an upgrade.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_archive_item** | [**ClusterArchiveItem**](ClusterArchiveItem.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_assess_item**
>crate::models::Empty create_cluster_assess_item(ctx, cluster_assess_item)


Start upgrade assessment on cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_assess_item** | [**ClusterAssessItem**](ClusterAssessItem.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_commit_item**
>crate::models::Empty create_cluster_commit_item(ctx, cluster_commit_item)


Commit the upgrade of a cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_commit_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_firmware_assess_item**
>crate::models::Empty create_cluster_firmware_assess_item(ctx, cluster_firmware_assess_item)


Start firmware upgrade assessment on cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_firmware_assess_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_firmware_upgrade_item**
>crate::models::Empty create_cluster_firmware_upgrade_item(ctx, cluster_firmware_upgrade_item)


The settings necessary to start a firmware upgrade.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_firmware_upgrade_item** | [**ClusterFirmwareUpgradeItem**](ClusterFirmwareUpgradeItem.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_patch_abort_item**
>crate::models::Empty create_cluster_patch_abort_item(ctx, cluster_patch_abort_item)


Abort the previous action performed by the patch system.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_patch_abort_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_patch_patch**
>crate::models::CreateResponse create_cluster_patch_patch(ctx, cluster_patch_patch, optional)


Install a patch.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_patch_patch** | [**ClusterPatchPatch**](ClusterPatchPatch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cluster_patch_patch** | [**ClusterPatchPatch**](ClusterPatchPatch.md)|  | 
 **_override** | **bool**| Whether to ignore patch system validation and force the installation. | 
 **rolling** | **bool**| Whether to install the patch on one node at a time. Defaults to true. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_retry_last_action_item**
>crate::models::Empty create_cluster_retry_last_action_item(ctx, cluster_retry_last_action_item)


Retry the last upgrade action, in-case the previous attempt failed.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_retry_last_action_item** | [**ClusterRetryLastActionItem**](ClusterRetryLastActionItem.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_rollback_item**
>crate::models::Empty create_cluster_rollback_item(ctx, cluster_rollback_item)


Rollback the upgrade of a cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_rollback_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cluster_upgrade_item**
>crate::models::Empty create_cluster_upgrade_item(ctx, cluster_upgrade_item)


The settings necessary to start an upgrade.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_upgrade_item** | [**ClusterUpgradeItem**](ClusterUpgradeItem.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_hardware_start_item**
> create_hardware_start_item(ctx, hardware_start_item)


Start a hardware upgrade

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardware_start_item** | [**HardwareStartItem**](HardwareStartItem.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_hardware_stop_item**
> create_hardware_stop_item(ctx, hardware_stop_item)


Stop an in-progess hardware upgrade process

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardware_stop_item** | [**HardwareStopItem**](HardwareStopItem.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_cluster_patch_patch**
> delete_cluster_patch_patch(ctx, cluster_patch_patch_id, optional)


Uninstall a patch.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_patch_patch_id** | **String**| Uninstall a patch. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cluster_patch_patch_id** | **String**| Uninstall a patch. | 
 **_override** | **bool**| Whether to ignore patch system validation and force the uninstallation. | 
 **rolling** | **bool**| Whether to uninstall the patch on one node at a time. Defaults to true. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_firmware_progress**
>crate::models::ClusterFirmwareProgress get_cluster_firmware_progress(ctx, )


Cluster wide firmware upgrade status info.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterFirmwareProgress**](ClusterFirmwareProgress.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_firmware_status**
>crate::models::ClusterFirmwareStatus get_cluster_firmware_status(ctx, optional)


The firmware status for the cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **devices** | **bool**| Show devices. If false, this returns an empty list. Default is false. | 
 **package** | **bool**| Show package. If false, this returns an empty list.Default is false. | 

### Return type

[**::models::ClusterFirmwareStatus**](ClusterFirmwareStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_node**
>crate::models::ClusterNodes get_cluster_node(ctx, cluster_node_id)


The node details useful during an upgrade or assessment.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_node_id** | **i32**| The node details useful during an upgrade or assessment. | 

### Return type

[**::models::ClusterNodes**](ClusterNodes.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_nodes**
>crate::models::ClusterNodesExtended get_cluster_nodes(ctx, )


View information about nodes during an upgrade, rollback, or pre-upgrade assessment.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterNodesExtended**](ClusterNodesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_patch_patch**
>crate::models::ClusterPatchPatches get_cluster_patch_patch(ctx, cluster_patch_patch_id, optional)


View a single patch.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_patch_patch_id** | **String**| View a single patch. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cluster_patch_patch_id** | **String**| View a single patch. | 
 **local** | **bool**| Only view patch information on the local node. | 
 **location** | **String**| Path location of patch file. | 

### Return type

[**::models::ClusterPatchPatches**](ClusterPatchPatches.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hardware_status**
>crate::models::HardwareStatus get_hardware_status(ctx, )


View the status of hardware upgrades in progress

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::HardwareStatus**](HardwareStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_upgrade_cluster**
>crate::models::UpgradeCluster get_upgrade_cluster(ctx, )


Cluster wide upgrade status info.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::UpgradeCluster**](UpgradeCluster.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_cluster_patch_patches**
>crate::models::ClusterPatchPatchesExtended list_cluster_patch_patches(ctx, optional)


List all patches.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **location** | **String**| Path location of patch file. | 
 **local** | **bool**| Whether to view patches on the local node only. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::ClusterPatchPatchesExtended**](ClusterPatchPatchesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cluster_upgrade**
> update_cluster_upgrade(ctx, cluster_upgrade)


Add nodes to a running upgrade.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_upgrade** | [**ClusterUpgrade**](ClusterUpgrade.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

