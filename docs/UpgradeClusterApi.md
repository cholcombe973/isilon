# \UpgradeClusterApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nodes_node_patch_sync_item**](UpgradeClusterApi.md#create_nodes_node_patch_sync_item) | **Post** /platform/4/upgrade/cluster/nodes/{Lnn}/patch/sync | 
[**get_nodes_node_firmware_status**](UpgradeClusterApi.md#get_nodes_node_firmware_status) | **Get** /platform/3/upgrade/cluster/nodes/{Lnn}/firmware/status | 


# **create_nodes_node_patch_sync_item**
>crate::models::Empty create_nodes_node_patch_sync_item(ctx, nodes_node_patch_sync_item, lnn)


Retry any pending patch sync operations.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nodes_node_patch_sync_item** | [**Empty**](Empty.md)|  | 
  **lnn** | **i32**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nodes_node_firmware_status**
>crate::models::NodesNodeFirmwareStatus get_nodes_node_firmware_status(ctx, lnn, optional)


The firmware status for the node.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **i32**|  | 
 **devices** | **bool**| Show devices. If false, this returns an empty list. Default is false. | 
 **package** | **bool**| Show package. If false, this returns an empty list.Default is false. | 

### Return type

[**::models::NodesNodeFirmwareStatus**](NodesNodeFirmwareStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

