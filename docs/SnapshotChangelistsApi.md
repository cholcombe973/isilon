# \SnapshotChangelistsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_changelist_lin**](SnapshotChangelistsApi.md#get_changelist_lin) | **Get** /platform/1/snapshot/changelists/{Changelist}/lins/{ChangelistLinId} | 
[**get_changelist_lins**](SnapshotChangelistsApi.md#get_changelist_lins) | **Get** /platform/1/snapshot/changelists/{Changelist}/lins | 


# **get_changelist_lin**
> ::models::ChangelistLins get_changelist_lin(ctx, changelist_lin_id, changelist, optional)


Get a single entry from the changelist.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **changelist_lin_id** | **i32**| Get a single entry from the changelist. | 
  **changelist** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **changelist_lin_id** | **i32**| Get a single entry from the changelist. | 
 **changelist** | **String**|  | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::ChangelistLins**](ChangelistLins.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_changelist_lins**
> ::models::ChangelistLinsExtended get_changelist_lins(ctx, changelist, optional)


Get entries from a changelist.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **changelist** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **changelist** | **String**|  | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::ChangelistLinsExtended**](ChangelistLinsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

