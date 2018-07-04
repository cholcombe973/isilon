# \AuthProvidersApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ads_provider_controllers**](AuthProvidersApi.md#get_ads_provider_controllers) | **Get** /platform/3/auth/providers/ads/{Id}/controllers | 
[**get_ads_provider_domain**](AuthProvidersApi.md#get_ads_provider_domain) | **Get** /platform/3/auth/providers/ads/{Id}/domains/{AdsProviderDomainId} | 
[**get_ads_provider_domains**](AuthProvidersApi.md#get_ads_provider_domains) | **Get** /platform/3/auth/providers/ads/{Id}/domains | 
[**get_ads_provider_search**](AuthProvidersApi.md#get_ads_provider_search) | **Get** /platform/1/auth/providers/ads/{Id}/search | 


# **get_ads_provider_controllers**
> ::models::AdsProviderControllers get_ads_provider_controllers(ctx, id, optional)


List all ADS controllers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 
 **groupnet** | **String**| Groupnet identifier | 

### Return type

[**::models::AdsProviderControllers**](AdsProviderControllers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ads_provider_domain**
> ::models::AdsProviderDomains get_ads_provider_domain(ctx, ads_provider_domain_id, id)


Retrieve the ADS domain information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ads_provider_domain_id** | **String**| Retrieve the ADS domain information. | 
  **id** | **String**|  | 

### Return type

[**::models::AdsProviderDomains**](AdsProviderDomains.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ads_provider_domains**
> ::models::AdsProviderDomains get_ads_provider_domains(ctx, id, optional)


List all ADS domains.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::AdsProviderDomains**](AdsProviderDomains.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ads_provider_search**
> ::models::AdsProviderSearch get_ads_provider_search(ctx, id, optional)


Retrieve search results.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 
 **domain** | **String**| The domain to search in. | 
 **description** | **String**| The user or group description to search for. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **search_users** | **bool**| If true, search for users. | 
 **filter** | **String**| The LDAP filter to apply to the search. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **user** | **String**| The user name for the domain if untrusted. | 
 **password** | **String**| The password for the domain if untrusted. | 
 **search_groups** | **bool**| If true, search for groups. | 

### Return type

[**::models::AdsProviderSearch**](AdsProviderSearch.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

