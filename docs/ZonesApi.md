# \ZonesApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_zone**](ZonesApi.md#create_zone) | **Post** /platform/3/zones | 
[**delete_zone**](ZonesApi.md#delete_zone) | **Delete** /platform/3/zones/{ZoneId} | 
[**get_zone**](ZonesApi.md#get_zone) | **Get** /platform/3/zones/{ZoneId} | 
[**list_zones**](ZonesApi.md#list_zones) | **Get** /platform/3/zones | 
[**update_zone**](ZonesApi.md#update_zone) | **Put** /platform/3/zones/{ZoneId} | 


# **create_zone**
>crate::models::CreateResponse create_zone(ctx, zone)


Create a new access zone.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **zone** | [**ZoneCreateParams**](ZoneCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_zone**
> delete_zone(ctx, zone_id)


Delete the access zone.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **zone_id** | **i32**| Delete the access zone. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_zone**
>crate::models::Zones get_zone(ctx, zone_id)


Retrieve the access zone information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **zone_id** | **i32**| Retrieve the access zone information. | 

### Return type

[**::models::Zones**](Zones.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_zones**
>crate::models::ZonesExtended list_zones(ctx, )


List all access zones.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ZonesExtended**](ZonesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_zone**
> update_zone(ctx, zone, zone_id)


Modify the access zone. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **zone** | [**Zone**](Zone.md)|  | 
  **zone_id** | **i32**| Modify the access zone. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

