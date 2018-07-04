# \AuthGroupsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group_member**](AuthGroupsApi.md#create_group_member) | **Post** /platform/1/auth/groups/{Group}/members | 
[**delete_group_member**](AuthGroupsApi.md#delete_group_member) | **Delete** /platform/1/auth/groups/{Group}/members/{GroupMemberId} | 
[**list_group_members**](AuthGroupsApi.md#list_group_members) | **Get** /platform/1/auth/groups/{Group}/members | 


# **create_group_member**
> ::models::CreateResponse create_group_member(ctx, group_member, group, optional)


Add a member to the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_member** | [**AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md)|  | 
  **group** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_member** | [**AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md)|  | 
 **group** | **String**|  | 
 **zone** | **String**| Filter group members by zone. | 
 **provider** | **String**| Filter group members by provider. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_group_member**
> delete_group_member(ctx, group_member_id, group, optional)


Remove the member from the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_member_id** | **String**| Remove the member from the group. | 
  **group** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_member_id** | **String**| Remove the member from the group. | 
 **group** | **String**|  | 
 **zone** | **String**| Filter group members by zone. | 
 **provider** | **String**| Filter group members by provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_group_members**
> ::models::GroupMembers list_group_members(ctx, group, optional)


List all the members of the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group** | **String**|  | 
 **resolve_names** | **bool**| Resolve names of personas. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **zone** | **String**| Filter group members by zone. | 
 **provider** | **String**| Filter group members by provider. | 

### Return type

[**::models::GroupMembers**](GroupMembers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

