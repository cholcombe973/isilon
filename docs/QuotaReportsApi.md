# \QuotaReportsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_report_about**](QuotaReportsApi.md#get_report_about) | **Get** /platform/1/quota/reports/{Rid}/about | 


# **get_report_about**
>crate::models::ReportAbout get_report_about(ctx, rid)


Retrieve report meta-data information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rid** | **String**|  | 

### Return type

[**::models::ReportAbout**](ReportAbout.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

