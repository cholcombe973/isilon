# \AuthUsersApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_member_of_item**](AuthUsersApi.md#create_user_member_of_item) | **Post** /platform/3/auth/users/{User}/member-of | 
[**delete_user_member_of_member_of**](AuthUsersApi.md#delete_user_member_of_member_of) | **Delete** /platform/3/auth/users/{User}/member-of/{UserMemberOfMemberOf} | 
[**list_user_member_of**](AuthUsersApi.md#list_user_member_of) | **Get** /platform/3/auth/users/{User}/member-of | 
[**update_user_change_password**](AuthUsersApi.md#update_user_change_password) | **Put** /platform/3/auth/users/{User}/change-password | 


# **create_user_member_of_item**
>crate::models::CreateResponse create_user_member_of_item(ctx, user_member_of_item, user, optional)


Add the user to a group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_member_of_item** | [**AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md)|  | 
  **user** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_member_of_item** | [**AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md)|  | 
 **user** | **String**|  | 
 **zone** | **String**| Filter groups by zone. | 
 **provider** | **String**| Filter groups by provider. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_user_member_of_member_of**
> delete_user_member_of_member_of(ctx, user_member_of_member_of, user, optional)


Remove the user from the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_member_of_member_of** | **String**| Remove the user from the group. | 
  **user** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_member_of_member_of** | **String**| Remove the user from the group. | 
 **user** | **String**|  | 
 **zone** | **String**| Filter groups by zone. | 
 **provider** | **String**| Filter groups by provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_user_member_of**
>crate::models::UserMemberOf list_user_member_of(ctx, user, optional)


List all groups the user is a member of.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user** | **String**|  | 
 **resolve_names** | **bool**| Resolve names of personas. | 
 **zone** | **String**| Filter groups by zone. | 
 **provider** | **String**| Filter groups by provider. | 

### Return type

[**::models::UserMemberOf**](UserMemberOf.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_user_change_password**
> update_user_change_password(ctx, user_change_password, user, optional)


Change the user's password.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_change_password** | [**UserChangePassword**](UserChangePassword.md)|  | 
  **user** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_change_password** | [**UserChangePassword**](UserChangePassword.md)|  | 
 **user** | **String**|  | 
 **zone** | **String**| Specifies access zone containing user. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

