# \DedupeApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dedupe_dedupe_summary**](DedupeApi.md#get_dedupe_dedupe_summary) | **Get** /platform/1/dedupe/dedupe-summary | 
[**get_dedupe_report**](DedupeApi.md#get_dedupe_report) | **Get** /platform/1/dedupe/reports/{DedupeReportId} | 
[**get_dedupe_reports**](DedupeApi.md#get_dedupe_reports) | **Get** /platform/1/dedupe/reports | 
[**get_dedupe_settings**](DedupeApi.md#get_dedupe_settings) | **Get** /platform/1/dedupe/settings | 
[**update_dedupe_settings**](DedupeApi.md#update_dedupe_settings) | **Put** /platform/1/dedupe/settings | 


# **get_dedupe_dedupe_summary**
> ::models::DedupeDedupeSummary get_dedupe_dedupe_summary(ctx, )


Return summary information about dedupe.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DedupeDedupeSummary**](DedupeDedupeSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dedupe_report**
> ::models::DedupeReports get_dedupe_report(ctx, dedupe_report_id, optional)


Retrieve a report for a single dedupe job.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dedupe_report_id** | **String**| Retrieve a report for a single dedupe job. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dedupe_report_id** | **String**| Retrieve a report for a single dedupe job. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::DedupeReports**](DedupeReports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dedupe_reports**
> ::models::DedupeReportsExtended get_dedupe_reports(ctx, optional)


List dedupe reports.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **begin** | **i32**| Restrict the query to reports at or after the given time, in seconds since the Epoch. | 
 **end** | **i32**| Restrict the query to reports at or before the given time, in seconds since the Epoch. | 
 **job_id** | **i32**| Restrict the query to the given job ID. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **job_type** | **String**| Restrict the query to the given job type. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::DedupeReportsExtended**](DedupeReportsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dedupe_settings**
> ::models::DedupeSettings get_dedupe_settings(ctx, )


Retrieve the dedupe settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DedupeSettings**](DedupeSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_dedupe_settings**
> update_dedupe_settings(ctx, dedupe_settings)


Modify the dedupe settings. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dedupe_settings** | [**DedupeSettingsExtended**](DedupeSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

