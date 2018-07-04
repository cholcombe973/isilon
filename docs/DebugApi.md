# \DebugApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_debug_stats**](DebugApi.md#delete_debug_stats) | **Delete** /platform/1/debug/stats | 
[**get_debug_stats**](DebugApi.md#get_debug_stats) | **Get** /platform/1/debug/stats | 


# **delete_debug_stats**
> delete_debug_stats(ctx, )


Clear per-resource statistics counters.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_debug_stats**
> ::models::DebugStats get_debug_stats(ctx, )


List cumulative call statistics for each resource.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DebugStats**](DebugStats.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

