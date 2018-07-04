# \ZonesSummaryApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_zones_summary**](ZonesSummaryApi.md#get_zones_summary) | **Get** /platform/1/zones-summary | 
[**get_zones_summary_zone**](ZonesSummaryApi.md#get_zones_summary_zone) | **Get** /platform/1/zones-summary/{ZonesSummaryZone} | 


# **get_zones_summary**
> ::models::ZonesSummaryExtended get_zones_summary(ctx, optional)


Retrieve access zone summary information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet** | **String**| Name of groupnet in which to list zones. | 

### Return type

[**::models::ZonesSummaryExtended**](ZonesSummaryExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_zones_summary_zone**
> ::models::ZonesSummary get_zones_summary_zone(ctx, zones_summary_zone)


Retrieve non-privileged access zone information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **zones_summary_zone** | **i32**| Retrieve non-privileged access zone information. | 

### Return type

[**::models::ZonesSummary**](ZonesSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

