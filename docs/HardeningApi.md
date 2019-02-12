# \HardeningApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_hardening_apply_item**](HardeningApi.md#create_hardening_apply_item) | **Post** /platform/3/hardening/apply | 
[**create_hardening_resolve_item**](HardeningApi.md#create_hardening_resolve_item) | **Post** /platform/3/hardening/resolve | 
[**create_hardening_revert_item**](HardeningApi.md#create_hardening_revert_item) | **Post** /platform/3/hardening/revert | 
[**get_hardening_state**](HardeningApi.md#get_hardening_state) | **Get** /platform/3/hardening/state | 
[**get_hardening_status**](HardeningApi.md#get_hardening_status) | **Get** /platform/3/hardening/status | 


# **create_hardening_apply_item**
>crate::models::CreateHardeningApplyItemResponse create_hardening_apply_item(ctx, hardening_apply_item)


Apply hardening on the cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardening_apply_item** | [**HardeningApplyItem**](HardeningApplyItem.md)|  | 

### Return type

[**::models::CreateHardeningApplyItemResponse**](CreateHardeningApplyItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_hardening_resolve_item**
>crate::models::CreateHardeningResolveItemResponse create_hardening_resolve_item(ctx, hardening_resolve_item, optional)


Resolve issues related to hardening, found in current cluster configuration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardening_resolve_item** | [**HardeningResolveItem**](HardeningResolveItem.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hardening_resolve_item** | [**HardeningResolveItem**](HardeningResolveItem.md)|  | 
 **accept** | **bool**| If true, execution proceeds to resolve all issues. If false, executrion aborts. This is a required argument. | 

### Return type

[**::models::CreateHardeningResolveItemResponse**](CreateHardeningResolveItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_hardening_revert_item**
>crate::models::CreateHardeningRevertItemResponse create_hardening_revert_item(ctx, hardening_revert_item, optional)


Revert hardening on the cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardening_revert_item** | [**Empty**](Empty.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hardening_revert_item** | [**Empty**](Empty.md)|  | 
 **force** | **bool**| If specified, revert operation continues even in case of a failure. Default is false in which case revert stops at the first failure. | 

### Return type

[**::models::CreateHardeningRevertItemResponse**](CreateHardeningRevertItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hardening_state**
>crate::models::HardeningState get_hardening_state(ctx, )


Get the state of the current hardening operation, if one is happening.  Note that this is different from the /status resource, which returns the overall hardening status of the cluster.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::HardeningState**](HardeningState.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hardening_status**
>crate::models::HardeningStatus get_hardening_status(ctx, )


Get a message indicating whether or not the cluster is hardened. Note that this is different from the /state resource, which returns the state of a specific hardening operation (apply or revert).

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::HardeningStatus**](HardeningStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

