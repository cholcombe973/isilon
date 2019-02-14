# \SyncReportsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_report_subreport**](SyncReportsApi.md#get_report_subreport) | **Get** /platform/4/sync/reports/{Rid}/subreports/{ReportSubreportId} | 
[**get_report_subreports**](SyncReportsApi.md#get_report_subreports) | **Get** /platform/4/sync/reports/{Rid}/subreports | 


# **get_report_subreport**
>crate::models::ReportSubreports get_report_subreport(ctx, report_subreport_id, rid)


View a single SyncIQ subreport.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **report_subreport_id** | **String**| View a single SyncIQ subreport. | 
  **rid** | **String**|  | 

### Return type

[**::models::ReportSubreports**](ReportSubreports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_report_subreports**
>crate::models::ReportSubreportsExtended get_report_subreports(ctx, rid, optional)


Get a list of SyncIQ subreports for a report.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rid** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rid** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **newer_than** | **i32**| Filter the returned reports to include only those whose jobs started more recently than the specified number of days ago. | 
 **state** | **String**| Filter the returned reports to include only those whose jobs are in this state. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::ReportSubreportsExtended**](ReportSubreportsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

