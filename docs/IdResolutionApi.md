# \IdResolutionApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_id_resolution_path**](IdResolutionApi.md#get_id_resolution_path) | **Get** /platform/4/id-resolution/paths/{IdResolutionPathId} | 
[**get_id_resolution_paths**](IdResolutionApi.md#get_id_resolution_paths) | **Get** /platform/4/id-resolution/paths | 


# **get_id_resolution_path**
>crate::models::IdResolutionPaths get_id_resolution_path(ctx, id_resolution_path_id)


List lin to path mappings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id_resolution_path_id** | **i32**| List lin to path mappings. | 

### Return type

[**::models::IdResolutionPaths**](IdResolutionPaths.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_id_resolution_paths**
>crate::models::IdResolutionPathsExtended get_id_resolution_paths(ctx, optional)


List lin to path mappings.

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
 **lins** | **String**| A comma separated list specifying the lins that will be mapped with a path. Only the lins specified in this list will be mapped. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::IdResolutionPathsExtended**](IdResolutionPathsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

