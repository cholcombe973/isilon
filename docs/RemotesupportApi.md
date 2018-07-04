# \RemotesupportApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_remotesupport_connectemc**](RemotesupportApi.md#get_remotesupport_connectemc) | **Get** /platform/1/remotesupport/connectemc | 
[**update_remotesupport_connectemc**](RemotesupportApi.md#update_remotesupport_connectemc) | **Put** /platform/1/remotesupport/connectemc | 


# **get_remotesupport_connectemc**
> ::models::RemotesupportConnectemc get_remotesupport_connectemc(ctx, )


List all settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::RemotesupportConnectemc**](RemotesupportConnectemc.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_remotesupport_connectemc**
> update_remotesupport_connectemc(ctx, remotesupport_connectemc)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **remotesupport_connectemc** | [**RemotesupportConnectemcConnectemc**](RemotesupportConnectemcConnectemc.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

