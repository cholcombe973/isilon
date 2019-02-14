# \WormApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_worm_domain**](WormApi.md#create_worm_domain) | **Post** /platform/1/worm/domains | 
[**get_worm_domain**](WormApi.md#get_worm_domain) | **Get** /platform/1/worm/domains/{WormDomainId} | 
[**get_worm_settings**](WormApi.md#get_worm_settings) | **Get** /platform/1/worm/settings | 
[**list_worm_domains**](WormApi.md#list_worm_domains) | **Get** /platform/1/worm/domains | 
[**update_worm_domain**](WormApi.md#update_worm_domain) | **Put** /platform/1/worm/domains/{WormDomainId} | 
[**update_worm_settings**](WormApi.md#update_worm_settings) | **Put** /platform/1/worm/settings | 


# **create_worm_domain**
>crate::models::WormDomainExtended create_worm_domain(ctx, worm_domain)


Create a WORM domain.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **worm_domain** | [**WormDomainCreateParams**](WormDomainCreateParams.md)|  | 

### Return type

[**::models::WormDomainExtended**](WormDomainExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_worm_domain**
>crate::models::WormDomains get_worm_domain(ctx, worm_domain_id)


View a single WORM domain.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **worm_domain_id** | **String**| View a single WORM domain. | 

### Return type

[**::models::WormDomains**](WormDomains.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_worm_settings**
>crate::models::WormSettings get_worm_settings(ctx, )


Get the global WORM settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::WormSettings**](WormSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_worm_domains**
>crate::models::WormDomainsExtended list_worm_domains(ctx, optional)


List all WORM domains.

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
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::WormDomainsExtended**](WormDomainsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_worm_domain**
> update_worm_domain(ctx, worm_domain, worm_domain_id)


Modify a single WORM domain.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **worm_domain** | [**WormDomain**](WormDomain.md)|  | 
  **worm_domain_id** | **String**| Modify a single WORM domain. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_worm_settings**
> update_worm_settings(ctx, worm_settings)


Modify the global WORM settings.  All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **worm_settings** | [**WormSettingsExtended**](WormSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

