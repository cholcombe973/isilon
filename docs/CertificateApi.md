# \CertificateApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_certificate_server_item**](CertificateApi.md#create_certificate_server_item) | **Post** /platform/4/certificate/server | 
[**delete_certificate_server_by_id**](CertificateApi.md#delete_certificate_server_by_id) | **Delete** /platform/4/certificate/server/{CertificateServerId} | 
[**get_certificate_server_by_id**](CertificateApi.md#get_certificate_server_by_id) | **Get** /platform/4/certificate/server/{CertificateServerId} | 
[**list_certificate_server**](CertificateApi.md#list_certificate_server) | **Get** /platform/4/certificate/server | 
[**update_certificate_server_by_id**](CertificateApi.md#update_certificate_server_by_id) | **Put** /platform/4/certificate/server/{CertificateServerId} | 


# **create_certificate_server_item**
>crate::models::CreateResponse create_certificate_server_item(ctx, certificate_server_item)


Import a TLS server certificate.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **certificate_server_item** | [**CertificateServerItem**](CertificateServerItem.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_certificate_server_by_id**
> delete_certificate_server_by_id(ctx, certificate_server_id)


Delete a TLS server certificate.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **certificate_server_id** | **String**| Delete a TLS server certificate. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_certificate_server_by_id**
>crate::models::CertificateServer get_certificate_server_by_id(ctx, certificate_server_id)


Retrieve a single TLS server certificate.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **certificate_server_id** | **String**| Retrieve a single TLS server certificate. | 

### Return type

[**::models::CertificateServer**](CertificateServer.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_certificate_server**
>crate::models::CertificateServerExtended list_certificate_server(ctx, optional)


Retrieve a list of all configured TLS server certificates.

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

[**::models::CertificateServerExtended**](CertificateServerExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_certificate_server_by_id**
> update_certificate_server_by_id(ctx, certificate_server_id_params, certificate_server_id)


Modify a TLS server certificate.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **certificate_server_id_params** | [**CertificateServerIdParams**](CertificateServerIdParams.md)|  | 
  **certificate_server_id** | **String**| Modify a TLS server certificate. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

