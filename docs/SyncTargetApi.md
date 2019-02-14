# \SyncTargetApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policies_policy_cancel_item**](SyncTargetApi.md#create_policies_policy_cancel_item) | **Post** /platform/1/sync/target/policies/{Policy}/cancel | 
[**get_reports_report_subreport**](SyncTargetApi.md#get_reports_report_subreport) | **Get** /platform/4/sync/target/reports/{Rid}/subreports/{ReportsReportSubreportId} | 
[**get_reports_report_subreports**](SyncTargetApi.md#get_reports_report_subreports) | **Get** /platform/4/sync/target/reports/{Rid}/subreports | 


# **create_policies_policy_cancel_item**
>crate::models::CreateResponse create_policies_policy_cancel_item(ctx, policies_policy_cancel_item, policy)


Cancel the most recent SyncIQ job for this policy from the target side.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policies_policy_cancel_item** | [**Empty**](Empty.md)|  | 
  **policy** | **String**|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_reports_report_subreport**
>crate::models::ReportsReportSubreports get_reports_report_subreport(ctx, reports_report_subreport_id, rid)


View a single SyncIQ target subreport.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **reports_report_subreport_id** | **String**| View a single SyncIQ target subreport. | 
  **rid** | **String**|  | 

### Return type

[**::models::ReportsReportSubreports**](ReportsReportSubreports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_reports_report_subreports**
>crate::models::ReportsReportSubreportsExtended get_reports_report_subreports(ctx, rid, optional)


Get a list of SyncIQ target subreports for a report.

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

[**::models::ReportsReportSubreportsExtended**](ReportsReportSubreportsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

