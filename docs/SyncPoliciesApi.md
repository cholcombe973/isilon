# \SyncPoliciesApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy_reset_item**](SyncPoliciesApi.md#create_policy_reset_item) | **Post** /platform/1/sync/policies/{Policy}/reset | 


# **create_policy_reset_item**
>crate::models::CreateResponse create_policy_reset_item(ctx, policy_reset_item, policy)


Reset a SyncIQ policy incremental state and force a full sync/copy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_reset_item** | [**Empty**](Empty.md)|  | 
  **policy** | **String**|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

