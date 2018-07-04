# \FsaApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_fsa_result**](FsaApi.md#delete_fsa_result) | **Delete** /platform/3/fsa/results/{FsaResultId} | 
[**delete_fsa_settings**](FsaApi.md#delete_fsa_settings) | **Delete** /platform/1/fsa/settings | 
[**get_fsa_result**](FsaApi.md#get_fsa_result) | **Get** /platform/3/fsa/results/{FsaResultId} | 
[**get_fsa_results**](FsaApi.md#get_fsa_results) | **Get** /platform/3/fsa/results | 
[**get_fsa_settings**](FsaApi.md#get_fsa_settings) | **Get** /platform/1/fsa/settings | 
[**update_fsa_result**](FsaApi.md#update_fsa_result) | **Put** /platform/3/fsa/results/{FsaResultId} | 
[**update_fsa_settings**](FsaApi.md#update_fsa_settings) | **Put** /platform/1/fsa/settings | 


# **delete_fsa_result**
> delete_fsa_result(ctx, fsa_result_id)


Delete the result set.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **fsa_result_id** | **String**| Delete the result set. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_fsa_settings**
> delete_fsa_settings(ctx, )


Revert all settings to their defaults.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fsa_result**
> ::models::FsaResults get_fsa_result(ctx, fsa_result_id)


Retrieve result set information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **fsa_result_id** | **String**| Retrieve result set information. | 

### Return type

[**::models::FsaResults**](FsaResults.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fsa_results**
> ::models::FsaResultsExtended get_fsa_results(ctx, )


List all results.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::FsaResultsExtended**](FsaResultsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fsa_settings**
> ::models::FsaSettings get_fsa_settings(ctx, optional)


List all settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as effective or not specified, all fields are returned.  If specified as user, only fields with non-default values are shown.  If specified as default, the original values are returned. | 

### Return type

[**::models::FsaSettings**](FsaSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_fsa_result**
> update_fsa_result(ctx, fsa_result, fsa_result_id)


Modify result set. Only the pinned property can be changed at this time.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **fsa_result** | [**FsaResult**](FsaResult.md)|  | 
  **fsa_result_id** | **String**| Modify result set. Only the pinned property can be changed at this time. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_fsa_settings**
> update_fsa_settings(ctx, fsa_settings)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **fsa_settings** | [**FsaSettingsSettings**](FsaSettingsSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

