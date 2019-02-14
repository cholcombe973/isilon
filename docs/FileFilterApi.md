# \FileFilterApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_file_filter_settings**](FileFilterApi.md#get_file_filter_settings) | **Get** /platform/3/file-filter/settings | 
[**update_file_filter_settings**](FileFilterApi.md#update_file_filter_settings) | **Put** /platform/3/file-filter/settings | 


# **get_file_filter_settings**
>crate::models::FileFilterSettings get_file_filter_settings(ctx, optional)


View File Filtering settings of an access zone

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Specifies the access zones in which these settings apply. | 

### Return type

[**::models::FileFilterSettings**](FileFilterSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_file_filter_settings**
> update_file_filter_settings(ctx, file_filter_settings, optional)


Modify one or more File Filtering settings for an access zone

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_filter_settings** | [**FileFilterSettingsExtended**](FileFilterSettingsExtended.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_filter_settings** | [**FileFilterSettingsExtended**](FileFilterSettingsExtended.md)|  | 
 **zone** | **String**| Specifies the access zones in which these settings apply. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

