# \JobApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_job_job**](JobApi.md#create_job_job) | **Post** /platform/3/job/jobs | 
[**create_job_policy**](JobApi.md#create_job_policy) | **Post** /platform/1/job/policies | 
[**delete_job_policy**](JobApi.md#delete_job_policy) | **Delete** /platform/1/job/policies/{JobPolicyId} | 
[**get_job_events**](JobApi.md#get_job_events) | **Get** /platform/3/job/events | 
[**get_job_job**](JobApi.md#get_job_job) | **Get** /platform/3/job/jobs/{JobJobId} | 
[**get_job_job_summary**](JobApi.md#get_job_job_summary) | **Get** /platform/1/job/job-summary | 
[**get_job_policy**](JobApi.md#get_job_policy) | **Get** /platform/1/job/policies/{JobPolicyId} | 
[**get_job_recent**](JobApi.md#get_job_recent) | **Get** /platform/3/job/recent | 
[**get_job_reports**](JobApi.md#get_job_reports) | **Get** /platform/3/job/reports | 
[**get_job_statistics**](JobApi.md#get_job_statistics) | **Get** /platform/1/job/statistics | 
[**get_job_type**](JobApi.md#get_job_type) | **Get** /platform/1/job/types/{JobTypeId} | 
[**get_job_types**](JobApi.md#get_job_types) | **Get** /platform/1/job/types | 
[**list_job_jobs**](JobApi.md#list_job_jobs) | **Get** /platform/3/job/jobs | 
[**list_job_policies**](JobApi.md#list_job_policies) | **Get** /platform/1/job/policies | 
[**update_job_job**](JobApi.md#update_job_job) | **Put** /platform/3/job/jobs/{JobJobId} | 
[**update_job_policy**](JobApi.md#update_job_policy) | **Put** /platform/1/job/policies/{JobPolicyId} | 
[**update_job_type**](JobApi.md#update_job_type) | **Put** /platform/1/job/types/{JobTypeId} | 


# **create_job_job**
> ::models::CreateJobJobResponse create_job_job(ctx, job_job)


Queue a new instance of a job type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_job** | [**JobJobCreateParams**](JobJobCreateParams.md)|  | 

### Return type

[**::models::CreateJobJobResponse**](CreateJobJobResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_job_policy**
> ::models::CreateResponse create_job_policy(ctx, job_policy)


Create a new job impact policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_policy** | [**JobPolicyCreateParams**](JobPolicyCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_job_policy**
> delete_job_policy(ctx, job_policy_id)


Delete a job impact policy.  System policies may not be deleted.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_policy_id** | **String**| Delete a job impact policy.  System policies may not be deleted. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_events**
> ::models::JobEvents get_job_events(ctx, optional)


List job events.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **begin** | **i32**| Restrict the query to events at or after the given time, in seconds since the Epoch. | 
 **end** | **i32**| Restrict the query to events before the given time, in seconds since the Epoch. | 
 **job_id** | **i32**| Restrict the query to the given job ID. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **job_type** | **String**| Restrict the query to the given job type. | 
 **timeout_ms** | **i32**| Query timeout in milliseconds. The default is 10000 ms. | 
 **state** | **String**| Restrict the query to events containing the given state. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **key** | **String**| Restrict the query to the given key name. | 

### Return type

[**::models::JobEvents**](JobEvents.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_job**
> ::models::JobJobs get_job_job(ctx, job_job_id)


View a single job instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_job_id** | **String**| View a single job instance. | 

### Return type

[**::models::JobJobs**](JobJobs.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_job_summary**
> ::models::JobJobSummary get_job_job_summary(ctx, )


View job engine status.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::JobJobSummary**](JobJobSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_policy**
> ::models::JobPolicies get_job_policy(ctx, job_policy_id)


View a single job impact policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_policy_id** | **String**| View a single job impact policy. | 

### Return type

[**::models::JobPolicies**](JobPolicies.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_recent**
> ::models::JobRecent get_job_recent(ctx, optional)


List recently completed jobs.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **timeout_ms** | **i32**| Query timeout in milliseconds. The default is 10000 ms. | 
 **limit** | **i32**| Max number of recent jobs to return. The default is 8, the max is 100. | 

### Return type

[**::models::JobRecent**](JobRecent.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_reports**
> ::models::JobReports get_job_reports(ctx, optional)


List job reports.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **begin** | **i32**| Restrict the query to reports at or after the given time, in seconds since the Epoch. | 
 **end** | **i32**| Restrict the query to reports before the given time, in seconds since the Epoch. | 
 **job_id** | **i32**| Restrict the query to the given job ID. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **job_type** | **String**| Restrict the query to the given job type. | 
 **timeout_ms** | **i32**| Query timeout in milliseconds. The default is 10000 ms. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **key** | **String**| Restrict the query to the given report key. | 
 **verbose** | **bool**| Display more detailed information, including job engine framework statistics. | 

### Return type

[**::models::JobReports**](JobReports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_statistics**
> ::models::JobStatistics get_job_statistics(ctx, optional)


View job engine statistics.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **devid** | **i32**| Restrict the query to the given node. | 
 **job_id** | **i32**| Restrict the query to the given job ID. | 

### Return type

[**::models::JobStatistics**](JobStatistics.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_type**
> ::models::JobTypes get_job_type(ctx, job_type_id)


Retrieve job type information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_type_id** | **String**| Retrieve job type information. | 

### Return type

[**::models::JobTypes**](JobTypes.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_job_types**
> ::models::JobTypesExtended get_job_types(ctx, optional)


List job types.

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
 **show_all** | **bool**| Whether to show all job types, including hidden ones.  Defaults to false. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::JobTypesExtended**](JobTypesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_job_jobs**
> ::models::JobJobsExtended list_job_jobs(ctx, optional)


List running and paused jobs.

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
 **batch** | **bool**| If true, other arguments are ignored, and the query will return all results, unsorted, as quickly as possible. | 
 **state** | **String**| Limit the results to jobs in the specified state. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::JobJobsExtended**](JobJobsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_job_policies**
> ::models::JobPoliciesExtended list_job_policies(ctx, optional)


List job impact policies.

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
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::JobPoliciesExtended**](JobPoliciesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_job_job**
> update_job_job(ctx, job_job, job_job_id)


Modify a running or paused job instance.  All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_job** | [**JobJob**](JobJob.md)|  | 
  **job_job_id** | **String**| Modify a running or paused job instance.  All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_job_policy**
> update_job_policy(ctx, job_policy, job_policy_id)


Modify a job impact policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_policy** | [**JobPolicy**](JobPolicy.md)|  | 
  **job_policy_id** | **String**| Modify a job impact policy. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_job_type**
> update_job_type(ctx, job_type, job_type_id)


Modify the job type.  All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_type** | [**JobType**](JobType.md)|  | 
  **job_type_id** | **String**| Modify the job type.  All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

