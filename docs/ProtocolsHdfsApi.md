# \ProtocolsHdfsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_proxyusers_name_member**](ProtocolsHdfsApi.md#create_proxyusers_name_member) | **Post** /platform/1/protocols/hdfs/proxyusers/{Name}/members | 
[**delete_proxyusers_name_member**](ProtocolsHdfsApi.md#delete_proxyusers_name_member) | **Delete** /platform/1/protocols/hdfs/proxyusers/{Name}/members/{ProxyusersNameMemberId} | 
[**list_proxyusers_name_members**](ProtocolsHdfsApi.md#list_proxyusers_name_members) | **Get** /platform/1/protocols/hdfs/proxyusers/{Name}/members | 
[**update_proxyusers_name_member**](ProtocolsHdfsApi.md#update_proxyusers_name_member) | **Put** /platform/1/protocols/hdfs/proxyusers/{Name}/members/{ProxyusersNameMemberId} | 


# **create_proxyusers_name_member**
>crate::models::Empty create_proxyusers_name_member(ctx, proxyusers_name_member, name, optional)


Add a member to the HDFS proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **proxyusers_name_member** | [**AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md)|  | 
  **name** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **proxyusers_name_member** | [**AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md)|  | 
 **name** | **String**|  | 
 **zone** | **String**| Specifies which access zone or zones to use. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_proxyusers_name_member**
> delete_proxyusers_name_member(ctx, proxyusers_name_member_id, name, optional)


Remove a member from the HDFS proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **proxyusers_name_member_id** | **String**| Remove a member from the HDFS proxyuser. | 
  **name** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **proxyusers_name_member_id** | **String**| Remove a member from the HDFS proxyuser. | 
 **name** | **String**|  | 
 **zone** | **String**| Specifies which access zone or zones to use. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_proxyusers_name_members**
>crate::models::GroupMembers list_proxyusers_name_members(ctx, name, optional)


List all the members of the HDFS proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**|  | 
 **zone** | **String**| Specifies which access zone or zones to use. | 

### Return type

[**::models::GroupMembers**](GroupMembers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_proxyusers_name_member**
> update_proxyusers_name_member(ctx, proxyusers_name_member, proxyusers_name_member_id, name, optional)


Create a new HDFS proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **proxyusers_name_member** | [**Empty**](Empty.md)|  | 
  **proxyusers_name_member_id** | **String**| Create a new HDFS proxyuser. | 
  **name** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **proxyusers_name_member** | [**Empty**](Empty.md)|  | 
 **proxyusers_name_member_id** | **String**| Create a new HDFS proxyuser. | 
 **name** | **String**|  | 
 **zone** | **String**| Specifies which access zone or zones to use. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

