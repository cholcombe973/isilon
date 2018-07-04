# \FsaResultsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_histogram_stat_by**](FsaResultsApi.md#get_histogram_stat_by) | **Get** /platform/3/fsa/results/{Id}/histogram/{Stat}/by | 
[**get_histogram_stat_by_breakout**](FsaResultsApi.md#get_histogram_stat_by_breakout) | **Get** /platform/3/fsa/results/{Id}/histogram/{Stat}/by/{HistogramStatByBreakout} | 
[**get_result_directories**](FsaResultsApi.md#get_result_directories) | **Get** /platform/3/fsa/results/{Id}/directories | 
[**get_result_directory**](FsaResultsApi.md#get_result_directory) | **Get** /platform/3/fsa/results/{Id}/directories/{ResultDirectoryId} | 
[**get_result_histogram**](FsaResultsApi.md#get_result_histogram) | **Get** /platform/3/fsa/results/{Id}/histogram | 
[**get_result_histogram_stat**](FsaResultsApi.md#get_result_histogram_stat) | **Get** /platform/3/fsa/results/{Id}/histogram/{ResultHistogramStat} | 
[**get_result_top_dir**](FsaResultsApi.md#get_result_top_dir) | **Get** /platform/3/fsa/results/{Id}/top-dirs/{ResultTopDirId} | 
[**get_result_top_dirs**](FsaResultsApi.md#get_result_top_dirs) | **Get** /platform/3/fsa/results/{Id}/top-dirs | 
[**get_result_top_file**](FsaResultsApi.md#get_result_top_file) | **Get** /platform/3/fsa/results/{Id}/top-files/{ResultTopFileId} | 
[**get_result_top_files**](FsaResultsApi.md#get_result_top_files) | **Get** /platform/3/fsa/results/{Id}/top-files | 


# **get_histogram_stat_by**
> ::models::HistogramStatBy get_histogram_stat_by(ctx, id, stat)


This resource retrieves a histogram breakout for an individual FSA result set. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
  **stat** | **String**|  | 

### Return type

[**::models::HistogramStatBy**](HistogramStatBy.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_histogram_stat_by_breakout**
> ::models::HistogramStatBy get_histogram_stat_by_breakout(ctx, histogram_stat_by_breakout, id, stat, optional)


This resource retrieves a histogram breakout for an individual FSA result set. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **histogram_stat_by_breakout** | **String**| This resource retrieves a histogram breakout for an individual FSA result set. ID in the resource path is the result set ID. | 
  **id** | **String**|  | 
  **stat** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **histogram_stat_by_breakout** | **String**| This resource retrieves a histogram breakout for an individual FSA result set. ID in the resource path is the result set ID. | 
 **id** | **String**|  | 
 **stat** | **String**|  | 
 **directory_filter** | **String**| Filter according to a specific directory, which includes all of its subdirectories. | 
 **attribute_filter** | **String**| Filter according to the name of a file user attribute. | 
 **node_pool_filter** | **String**| Filter according to the name of a node pool, which is a set of disk pools that belong to nodes of the same equivalence class. | 
 **disk_pool_filter** | **String**| Filter according to the name of a disk pool, which is a set of drives that represent an independent failure domain. | 
 **tier_filter** | **String**| Filter according to the name of a storage tier, which is a user-created set of node pools. | 
 **comp_report** | **i32**| Result set identifier for comparison of database results. | 
 **log_size_filter** | **i32**| Filter according to file logical size, where the filter value specifies the lower bound in bytes to a set of files that have been grouped by logical size. The list of valid log_size filter values may be found by performing a histogram breakout by log_size and viewing the resulting key values. | 
 **phys_size_filter** | **i32**| Filter according to file physical size, where the filter value specifies the lower bound in bytes to a set of files that have been grouped by physical size. The list of valid phys_size filter values may be found by performing a histogram breakout by phys_size and viewing the resulting key values. | 
 **limit** | **i32**| Limit the number of breakout results. | 
 **path_ext_filter** | **String**| Filter according to the name of a single file extension. | 
 **ctime_filter** | **i32**| Filter according to file modified time, where the filter value specifies a negative number of seconds representing a time before the begin time of the report. The list of valid ctime filter values may be found by performing a histogram breakout by ctime and viewing the resulting key values. | 
 **atime_filter** | **i32**| Filter according to file accessed time, where the filter value specifies a negative number of seconds representing a time before the begin time of the report. The list of valid atime filter values may be found by performing a histogram breakout by atime and viewing the resulting key values. | 

### Return type

[**::models::HistogramStatBy**](HistogramStatBy.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_directories**
> ::models::ResultDirectories get_result_directories(ctx, id, optional)


This resource retrieves directory information. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **path** | **String**| Primary directory path to report usage information, which may be specified instead of a LIN. | 
 **limit** | **i32**| Limit the number of reported subdirectories. | 
 **comp_report** | **i32**| Result set identifier for comparison of database results. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::ResultDirectories**](ResultDirectories.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_directory**
> ::models::ResultDirectories get_result_directory(ctx, result_directory_id, id, optional)


This resource retrieves directory information. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **result_directory_id** | **i32**| This resource retrieves directory information. ID in the resource path is the result set ID. | 
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **result_directory_id** | **i32**| This resource retrieves directory information. ID in the resource path is the result set ID. | 
 **id** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **limit** | **i32**| Limit the number of reported subdirectories. | 
 **comp_report** | **i32**| Result set identifier for comparison of database results. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::ResultDirectories**](ResultDirectories.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_histogram**
> ::models::ResultHistogram get_result_histogram(ctx, id)


This resource retrieves a histogram of file counts for an individual FSA result set. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 

### Return type

[**::models::ResultHistogram**](ResultHistogram.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_histogram_stat**
> ::models::ResultHistogram get_result_histogram_stat(ctx, result_histogram_stat, id, optional)


This resource retrieves a histogram of file counts for an individual FSA result set. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **result_histogram_stat** | **String**| This resource retrieves a histogram of file counts for an individual FSA result set. ID in the resource path is the result set ID. | 
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **result_histogram_stat** | **String**| This resource retrieves a histogram of file counts for an individual FSA result set. ID in the resource path is the result set ID. | 
 **id** | **String**|  | 
 **directory_filter** | **String**| Filter according to a specific directory, which includes all of its subdirectories. | 
 **attribute_filter** | **String**| Filter according to the name of a file user attribute. | 
 **node_pool_filter** | **String**| Filter according to the name of a node pool, which is a set of disk pools that belong to nodes of the same equivalence class. | 
 **disk_pool_filter** | **String**| Filter according to the name of a disk pool, which is a set of drives that represent an independent failure domain. | 
 **tier_filter** | **String**| Filter according to the name of a storage tier, which is a user-created set of node pools. | 
 **comp_report** | **i32**| Result set identifier for comparison of database results. | 
 **log_size_filter** | **i32**| Filter according to file logical size, where the filter value specifies the lower bound in bytes to a set of files that have been grouped by logical size. The list of valid log_size filter values may be found by performing a histogram breakout by log_size and viewing the resulting key values. | 
 **phys_size_filter** | **i32**| Filter according to file physical size, where the filter value specifies the lower bound in bytes to a set of files that have been grouped by physical size. The list of valid phys_size filter values may be found by performing a histogram breakout by phys_size and viewing the resulting key values. | 
 **path_ext_filter** | **String**| Filter according to the name of a single file extension. | 
 **ctime_filter** | **i32**| Filter according to file modified time, where the filter value specifies a negative number of seconds representing a time before the begin time of the report. The list of valid ctime filter values may be found by performing a histogram breakout by ctime and viewing the resulting key values. | 
 **atime_filter** | **i32**| Filter according to file accessed time, where the filter value specifies a negative number of seconds representing a time before the begin time of the report. The list of valid atime filter values may be found by performing a histogram breakout by atime and viewing the resulting key values. | 

### Return type

[**::models::ResultHistogram**](ResultHistogram.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_top_dir**
> ::models::ResultTopDirs get_result_top_dir(ctx, result_top_dir_id, id, optional)


This resource retrieves the top directories. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **result_top_dir_id** | **String**| This resource retrieves the top directories. ID in the resource path is the result set ID. | 
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **result_top_dir_id** | **String**| This resource retrieves the top directories. ID in the resource path is the result set ID. | 
 **id** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **start** | **i32**| Starting index for results. Default value of 0. | 
 **limit** | **i32**| Number of results from start index. Default value of 1000. | 
 **comp_report** | **i32**| Result set identifier for comparison of database results. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::ResultTopDirs**](ResultTopDirs.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_top_dirs**
> ::models::ResultTopDirs get_result_top_dirs(ctx, id)


This resource retrieves the top directories. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 

### Return type

[**::models::ResultTopDirs**](ResultTopDirs.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_top_file**
> ::models::ResultTopFiles get_result_top_file(ctx, result_top_file_id, id, optional)


This resource retrieves the top files. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **result_top_file_id** | **String**| This resource retrieves the top files. ID in the resource path is the result set ID. | 
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **result_top_file_id** | **String**| This resource retrieves the top files. ID in the resource path is the result set ID. | 
 **id** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **start** | **i32**| Starting index for results. Default value of 0. | 
 **limit** | **i32**| Number of results from start index. Default value of 1000. | 
 **comp_report** | **i32**| Result set identifier for comparison of database results. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::ResultTopFiles**](ResultTopFiles.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_result_top_files**
> ::models::ResultTopFiles get_result_top_files(ctx, id)


This resource retrieves the top files. ID in the resource path is the result set ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 

### Return type

[**::models::ResultTopFiles**](ResultTopFiles.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

