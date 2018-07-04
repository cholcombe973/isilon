# \SyncApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sync_job**](SyncApi.md#create_sync_job) | **Post** /platform/3/sync/jobs | 
[**create_sync_policy**](SyncApi.md#create_sync_policy) | **Post** /platform/3/sync/policies | 
[**create_sync_reports_rotate_item**](SyncApi.md#create_sync_reports_rotate_item) | **Post** /platform/1/sync/reports-rotate | 
[**create_sync_rule**](SyncApi.md#create_sync_rule) | **Post** /platform/3/sync/rules | 
[**delete_sync_policies**](SyncApi.md#delete_sync_policies) | **Delete** /platform/3/sync/policies | 
[**delete_sync_policy**](SyncApi.md#delete_sync_policy) | **Delete** /platform/3/sync/policies/{SyncPolicyId} | 
[**delete_sync_rule**](SyncApi.md#delete_sync_rule) | **Delete** /platform/3/sync/rules/{SyncRuleId} | 
[**delete_sync_rules**](SyncApi.md#delete_sync_rules) | **Delete** /platform/3/sync/rules | 
[**delete_target_policy**](SyncApi.md#delete_target_policy) | **Delete** /platform/1/sync/target/policies/{TargetPolicyId} | 
[**get_history_cpu**](SyncApi.md#get_history_cpu) | **Get** /platform/3/sync/history/cpu | 
[**get_history_file**](SyncApi.md#get_history_file) | **Get** /platform/1/sync/history/file | 
[**get_history_network**](SyncApi.md#get_history_network) | **Get** /platform/1/sync/history/network | 
[**get_history_worker**](SyncApi.md#get_history_worker) | **Get** /platform/3/sync/history/worker | 
[**get_sync_job**](SyncApi.md#get_sync_job) | **Get** /platform/3/sync/jobs/{SyncJobId} | 
[**get_sync_license**](SyncApi.md#get_sync_license) | **Get** /platform/5/sync/license | 
[**get_sync_policy**](SyncApi.md#get_sync_policy) | **Get** /platform/3/sync/policies/{SyncPolicyId} | 
[**get_sync_report**](SyncApi.md#get_sync_report) | **Get** /platform/4/sync/reports/{SyncReportId} | 
[**get_sync_reports**](SyncApi.md#get_sync_reports) | **Get** /platform/4/sync/reports | 
[**get_sync_rule**](SyncApi.md#get_sync_rule) | **Get** /platform/3/sync/rules/{SyncRuleId} | 
[**get_sync_settings**](SyncApi.md#get_sync_settings) | **Get** /platform/3/sync/settings | 
[**get_target_policies**](SyncApi.md#get_target_policies) | **Get** /platform/1/sync/target/policies | 
[**get_target_policy**](SyncApi.md#get_target_policy) | **Get** /platform/1/sync/target/policies/{TargetPolicyId} | 
[**get_target_report**](SyncApi.md#get_target_report) | **Get** /platform/4/sync/target/reports/{TargetReportId} | 
[**get_target_reports**](SyncApi.md#get_target_reports) | **Get** /platform/4/sync/target/reports | 
[**list_sync_jobs**](SyncApi.md#list_sync_jobs) | **Get** /platform/3/sync/jobs | 
[**list_sync_policies**](SyncApi.md#list_sync_policies) | **Get** /platform/3/sync/policies | 
[**list_sync_reports_rotate**](SyncApi.md#list_sync_reports_rotate) | **Get** /platform/1/sync/reports-rotate | 
[**list_sync_rules**](SyncApi.md#list_sync_rules) | **Get** /platform/3/sync/rules | 
[**update_sync_job**](SyncApi.md#update_sync_job) | **Put** /platform/3/sync/jobs/{SyncJobId} | 
[**update_sync_policy**](SyncApi.md#update_sync_policy) | **Put** /platform/3/sync/policies/{SyncPolicyId} | 
[**update_sync_rule**](SyncApi.md#update_sync_rule) | **Put** /platform/3/sync/rules/{SyncRuleId} | 
[**update_sync_settings**](SyncApi.md#update_sync_settings) | **Put** /platform/3/sync/settings | 


# **create_sync_job**
> ::models::CreateResponse create_sync_job(ctx, sync_job)


Start a SyncIQ job.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_job** | [**SyncJobCreateParams**](SyncJobCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_sync_policy**
> ::models::CreateResponse create_sync_policy(ctx, sync_policy)


Create a SyncIQ policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_policy** | [**SyncPolicyCreateParams**](SyncPolicyCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_sync_reports_rotate_item**
> ::models::CreateSyncReportsRotateItemResponse create_sync_reports_rotate_item(ctx, sync_reports_rotate_item)


Rotate the records in the database(s).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_reports_rotate_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::CreateSyncReportsRotateItemResponse**](CreateSyncReportsRotateItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_sync_rule**
> ::models::CreateResponse create_sync_rule(ctx, sync_rule)


Create a new SyncIQ performance rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_rule** | [**SyncRuleCreateParams**](SyncRuleCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_sync_policies**
> delete_sync_policies(ctx, optional)


Delete all SyncIQ policies.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **local_only** | **bool**| Skip deleting the policy association on the target. | 
 **force** | **bool**| Ignore any running jobs when preparing to delete a policy. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_sync_policy**
> delete_sync_policy(ctx, sync_policy_id, optional)


Delete a single SyncIQ policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_policy_id** | **String**| Delete a single SyncIQ policy. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sync_policy_id** | **String**| Delete a single SyncIQ policy. | 
 **local_only** | **bool**| Skip deleting the policy association on the target. | 
 **force** | **bool**| Ignore any running jobs when preparing to delete a policy. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_sync_rule**
> delete_sync_rule(ctx, sync_rule_id)


Delete a single SyncIQ performance rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_rule_id** | **String**| Delete a single SyncIQ performance rule. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_sync_rules**
> delete_sync_rules(ctx, optional)


Delete all SyncIQ performance rules or all rules of a specified type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**| Delete all rules of the specified rule type only. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_target_policy**
> delete_target_policy(ctx, target_policy_id, optional)


Break the target association with this cluster for this policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **target_policy_id** | **String**| Break the target association with this cluster for this policy. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **target_policy_id** | **String**| Break the target association with this cluster for this policy. | 
 **force** | **bool**| Ignore any running jobs when preparing to delete the policy target association. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_history_cpu**
> ::models::HistoryFile get_history_cpu(ctx, optional)


List cpu performance data.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **begin** | **i32**| Begin timestamp for time-series report. | 
 **end** | **i32**| End timestamp for time-series report. | 

### Return type

[**::models::HistoryFile**](HistoryFile.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_history_file**
> ::models::HistoryFile get_history_file(ctx, optional)


List file operations performance data.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **begin** | **i32**| Begin timestamp for time-series report. | 
 **end** | **i32**| End timestamp for time-series report. | 

### Return type

[**::models::HistoryFile**](HistoryFile.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_history_network**
> ::models::HistoryFile get_history_network(ctx, optional)


List network operations performance data.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **begin** | **i32**| Begin timestamp for time-series report. | 
 **end** | **i32**| End timestamp for time-series report. | 

### Return type

[**::models::HistoryFile**](HistoryFile.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_history_worker**
> ::models::HistoryFile get_history_worker(ctx, optional)


List worker performance data.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **begin** | **i32**| Begin timestamp for time-series report. | 
 **end** | **i32**| End timestamp for time-series report. | 

### Return type

[**::models::HistoryFile**](HistoryFile.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_job**
> ::models::SyncJobs get_sync_job(ctx, sync_job_id)


View a single SyncIQ job.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_job_id** | **String**| View a single SyncIQ job. | 

### Return type

[**::models::SyncJobs**](SyncJobs.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_license**
> ::models::LicenseLicense get_sync_license(ctx, )


Retrieve license information.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::LicenseLicense**](LicenseLicense.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_policy**
> ::models::SyncPolicies get_sync_policy(ctx, sync_policy_id)


View a single SyncIQ policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_policy_id** | **String**| View a single SyncIQ policy. | 

### Return type

[**::models::SyncPolicies**](SyncPolicies.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_report**
> ::models::SyncReports get_sync_report(ctx, sync_report_id)


View a single SyncIQ report.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_report_id** | **String**| View a single SyncIQ report. | 

### Return type

[**::models::SyncReports**](SyncReports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_reports**
> ::models::SyncReportsExtended get_sync_reports(ctx, optional)


Get a list of SyncIQ reports.  By default 10 reports are returned per policy, unless otherwise specified by 'reports_per_policy'.

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
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **newer_than** | **i32**| Filter the returned reports to include only those whose jobs started more recently than the specified number of days ago. | 
 **policy_name** | **String**| Filter the returned reports to include only those with this policy name. | 
 **state** | **String**| Filter the returned reports to include only those whose jobs are in this state. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **reports_per_policy** | **i32**| If specified, only the N most recent reports will be returned per policy.  If no other query args are present this argument defaults to 10.  | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::SyncReportsExtended**](SyncReportsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_rule**
> ::models::SyncRules get_sync_rule(ctx, sync_rule_id)


View a single SyncIQ performance rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_rule_id** | **String**| View a single SyncIQ performance rule. | 

### Return type

[**::models::SyncRules**](SyncRules.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_settings**
> ::models::SyncSettings get_sync_settings(ctx, )


Retrieve the global SyncIQ settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SyncSettings**](SyncSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_target_policies**
> ::models::TargetPoliciesExtended get_target_policies(ctx, optional)


List all SyncIQ target policies.

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
 **target_path** | **String**| Filter the returned policies to include only those with this target path. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::TargetPoliciesExtended**](TargetPoliciesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_target_policy**
> ::models::TargetPolicies get_target_policy(ctx, target_policy_id)


View a single SyncIQ target policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **target_policy_id** | **String**| View a single SyncIQ target policy. | 

### Return type

[**::models::TargetPolicies**](TargetPolicies.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_target_report**
> ::models::TargetReports get_target_report(ctx, target_report_id)


View a single SyncIQ target report.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **target_report_id** | **String**| View a single SyncIQ target report. | 

### Return type

[**::models::TargetReports**](TargetReports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_target_reports**
> ::models::TargetReportsExtended get_target_reports(ctx, optional)


Get a list of SyncIQ target reports.  By default 10 reports are returned per policy, unless otherwise specified by 'reports_per_policy'.

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
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **newer_than** | **i32**| Filter the returned reports to include only those whose jobs started more recently than the specified number of days ago. | 
 **policy_name** | **String**| Filter the returned reports to include only those with this policy name. | 
 **state** | **String**| Filter the returned reports to include only those whose jobs are in this state. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **reports_per_policy** | **i32**| If specified, only the N most recent reports will be returned per policy.  If no other query args are present this argument defaults to 10.  | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::TargetReportsExtended**](TargetReportsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_sync_jobs**
> ::models::SyncJobsExtended list_sync_jobs(ctx, optional)


Get a list of SyncIQ jobs.

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
 **state** | **String**| The state of the job. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SyncJobsExtended**](SyncJobsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_sync_policies**
> ::models::SyncPoliciesExtended list_sync_policies(ctx, optional)


List all SyncIQ policies.

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
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **summary** | **bool**| Show only summary properties | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::SyncPoliciesExtended**](SyncPoliciesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_sync_reports_rotate**
> ::models::SyncReportsRotate list_sync_reports_rotate(ctx, )


Whether the rotation is still running or not.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SyncReportsRotate**](SyncReportsRotate.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_sync_rules**
> ::models::SyncRulesExtended list_sync_rules(ctx, optional)


List all SyncIQ performance rules.

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
 **_type** | **String**| Filter the returned rules to include only those with this rule type. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SyncRulesExtended**](SyncRulesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_sync_job**
> update_sync_job(ctx, sync_job, sync_job_id)


Perform an action (pause, cancel, etc...) on a single job.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_job** | [**SyncJob**](SyncJob.md)|  | 
  **sync_job_id** | **String**| Perform an action (pause, cancel, etc...) on a single job. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_sync_policy**
> update_sync_policy(ctx, sync_policy, sync_policy_id)


Modify a single SyncIQ policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_policy** | [**SyncPolicy**](SyncPolicy.md)|  | 
  **sync_policy_id** | **String**| Modify a single SyncIQ policy. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_sync_rule**
> update_sync_rule(ctx, sync_rule, sync_rule_id)


Modify a single SyncIQ performance rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_rule** | [**SyncRule**](SyncRule.md)|  | 
  **sync_rule_id** | **String**| Modify a single SyncIQ performance rule. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_sync_settings**
> update_sync_settings(ctx, sync_settings)


Modify the global SyncIQ settings.  All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sync_settings** | [**SyncSettingsExtended**](SyncSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

