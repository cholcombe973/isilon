# \FilepoolApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_filepool_policy**](FilepoolApi.md#create_filepool_policy) | **Post** /platform/4/filepool/policies | 
[**delete_filepool_policy**](FilepoolApi.md#delete_filepool_policy) | **Delete** /platform/4/filepool/policies/{FilepoolPolicyId} | 
[**get_filepool_default_policy**](FilepoolApi.md#get_filepool_default_policy) | **Get** /platform/4/filepool/default-policy | 
[**get_filepool_policy**](FilepoolApi.md#get_filepool_policy) | **Get** /platform/4/filepool/policies/{FilepoolPolicyId} | 
[**get_filepool_template**](FilepoolApi.md#get_filepool_template) | **Get** /platform/4/filepool/templates/{FilepoolTemplateId} | 
[**get_filepool_templates**](FilepoolApi.md#get_filepool_templates) | **Get** /platform/4/filepool/templates | 
[**list_filepool_policies**](FilepoolApi.md#list_filepool_policies) | **Get** /platform/4/filepool/policies | 
[**update_filepool_default_policy**](FilepoolApi.md#update_filepool_default_policy) | **Put** /platform/4/filepool/default-policy | 
[**update_filepool_policy**](FilepoolApi.md#update_filepool_policy) | **Put** /platform/4/filepool/policies/{FilepoolPolicyId} | 


# **create_filepool_policy**
>crate::models::CreateFilepoolPolicyResponse create_filepool_policy(ctx, filepool_policy)


Create a new policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **filepool_policy** | [**FilepoolPolicyCreateParams**](FilepoolPolicyCreateParams.md)|  | 

### Return type

[**::models::CreateFilepoolPolicyResponse**](CreateFilepoolPolicyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_filepool_policy**
> delete_filepool_policy(ctx, filepool_policy_id)


Delete file pool policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **filepool_policy_id** | **String**| Delete file pool policy. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_filepool_default_policy**
>crate::models::FilepoolDefaultPolicy get_filepool_default_policy(ctx, )


List default file pool policy.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::FilepoolDefaultPolicy**](FilepoolDefaultPolicy.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_filepool_policy**
>crate::models::FilepoolPolicies get_filepool_policy(ctx, filepool_policy_id)


Retrieve file pool policy information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **filepool_policy_id** | **String**| Retrieve file pool policy information. | 

### Return type

[**::models::FilepoolPolicies**](FilepoolPolicies.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_filepool_template**
>crate::models::FilepoolTemplates get_filepool_template(ctx, filepool_template_id)


List all templates.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **filepool_template_id** | **String**| List all templates. | 

### Return type

[**::models::FilepoolTemplates**](FilepoolTemplates.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_filepool_templates**
>crate::models::FilepoolTemplates get_filepool_templates(ctx, )


List all templates.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::FilepoolTemplates**](FilepoolTemplates.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_filepool_policies**
>crate::models::FilepoolPoliciesExtended list_filepool_policies(ctx, )


List all policies.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::FilepoolPoliciesExtended**](FilepoolPoliciesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_filepool_default_policy**
> update_filepool_default_policy(ctx, filepool_default_policy)


Set default file pool policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **filepool_default_policy** | [**FilepoolDefaultPolicyExtended**](FilepoolDefaultPolicyExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_filepool_policy**
> update_filepool_policy(ctx, filepool_policy, filepool_policy_id)


Modify file pool policy. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **filepool_policy** | [**FilepoolPolicy**](FilepoolPolicy.md)|  | 
  **filepool_policy_id** | **String**| Modify file pool policy. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

