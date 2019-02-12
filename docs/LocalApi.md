# \LocalApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cluster_time**](LocalApi.md#get_cluster_time) | **Get** /platform/3/local/cluster/time | 


# **get_cluster_time**
>crate::models::ClusterTimeExtendedExtended get_cluster_time(ctx, )


Get the current time on the local node.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterTimeExtendedExtended**](ClusterTimeExtendedExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

