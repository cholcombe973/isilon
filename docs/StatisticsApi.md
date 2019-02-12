# \StatisticsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_statistics_current**](StatisticsApi.md#get_statistics_current) | **Get** /platform/1/statistics/current | 
[**get_statistics_history**](StatisticsApi.md#get_statistics_history) | **Get** /platform/1/statistics/history | 
[**get_statistics_key**](StatisticsApi.md#get_statistics_key) | **Get** /platform/1/statistics/keys/{StatisticsKeyId} | 
[**get_statistics_keys**](StatisticsApi.md#get_statistics_keys) | **Get** /platform/1/statistics/keys | 
[**get_statistics_operations**](StatisticsApi.md#get_statistics_operations) | **Get** /platform/3/statistics/operations | 
[**get_statistics_protocols**](StatisticsApi.md#get_statistics_protocols) | **Get** /platform/1/statistics/protocols | 
[**get_summary_client**](StatisticsApi.md#get_summary_client) | **Get** /platform/3/statistics/summary/client | 
[**get_summary_drive**](StatisticsApi.md#get_summary_drive) | **Get** /platform/3/statistics/summary/drive | 
[**get_summary_heat**](StatisticsApi.md#get_summary_heat) | **Get** /platform/3/statistics/summary/heat | 
[**get_summary_protocol**](StatisticsApi.md#get_summary_protocol) | **Get** /platform/3/statistics/summary/protocol | 
[**get_summary_protocol_stats**](StatisticsApi.md#get_summary_protocol_stats) | **Get** /platform/3/statistics/summary/protocol-stats | 
[**get_summary_system**](StatisticsApi.md#get_summary_system) | **Get** /platform/3/statistics/summary/system | 
[**get_summary_workload**](StatisticsApi.md#get_summary_workload) | **Get** /platform/4/statistics/summary/workload | 


# **get_statistics_current**
>crate::models::StatisticsCurrent get_statistics_current(ctx, optional)


Retrieve stats.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **timeout** | **i32**| Time in seconds to wait for results from remote nodes. | 
 **show_nodes** | **bool**| Shows the logical node number or LNN in addition to the devid. | 
 **keys** | [**Vec&lt;String&gt;**](String.md)| Multiple key names. May request matching keys or request &#39;all&#39; keys. Can be comma separated list or can be used more than one time to make queries for multiple keys. May be used in conjunction with &#39;substr&#39;. Also works with &#39;key&#39; arguments. | 
 **devid** | [**Vec&lt;String&gt;**](String.md)| Node devid to query. Either an &lt;integer&gt; or \&quot;all\&quot;. Can be used more than one time to query multiple nodes. \&quot;all\&quot; queries all up nodes. 0 means query the local node. For \&quot;cluster\&quot; scoped keys, in any devid including 0 can be used. | 
 **substr** | **bool**| Used in conjunction with the &#39;keys&#39; argument, alters the behavior of keys. Makes the &#39;keys&#39; arg perform a partial match. Defaults to false. | 
 **stale** | **bool**| For internal use only, please do not use this. | 
 **type_info** | **bool**| The type ID is used by internal services. For internal use only, please do not use this. | 
 **raw** | **bool**| Causes the output to be in hex format. For internal use only, please do not use this. | 
 **key** | [**Vec&lt;String&gt;**](String.md)| One key name. Can be used more than one time to query multiple keys. Also works with &#39;keys&#39; arguments. | 
 **degraded** | **bool**| If true, try to continue even if some stats are unavailable. In this case, errors will be present in the per-key returned data. | 
 **nodes** | [**Vec&lt;String&gt;**](String.md)| Specify node(s) for which statistics should be reported. One or more comma separated &lt;integer(s)&gt; specifying which node(s) to query, or \&quot;all\&quot;. Specifying more than one node may have unspecified results for keys that begin with \&quot;cluster\&quot;. | 

### Return type

[**::models::StatisticsCurrent**](StatisticsCurrent.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_statistics_history**
>crate::models::StatisticsHistory get_statistics_history(ctx, optional)


Retrieve stats.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **begin** | **i32**| Earliest time (Unix epoch seconds) of interest. Negative times are interpreted as relative (before) now. | 
 **interval** | **i32**| Minimum sampling interval time in seconds. If native statistics are higher resolution, data will be down-sampled. | 
 **end** | **i32**| Latest time (Unix epoch seconds) of interest. Negative times are interpreted as relative (before) now. If not supplied, use now as the end time. | 
 **timeout** | **i32**| Time in seconds to wait for results from remote nodes. | 
 **raw** | **bool**| Causes the output to be in hex format. For internal use only, please do not use this. | 
 **keys** | [**Vec&lt;String&gt;**](String.md)| Multiple key names. May request matching keys or request &#39;all&#39; keys. Can be comma separated list or can be used more than one time to make queries for multiple keys. May be used in conjunction with &#39;substr&#39;. Also works with &#39;key&#39; arguments. | 
 **devid** | [**Vec&lt;String&gt;**](String.md)| Node devid to query. Either an &lt;integer&gt; or \&quot;all\&quot;. Can be used more than one time to query multiple nodes. \&quot;all\&quot; queries all up nodes. 0 means query the local node. For \&quot;cluster\&quot; scoped keys, in any devid including 0 can be used. | 
 **substr** | **bool**| Used in conjunction with the &#39;keys&#39; argument, alters the behavior of keys. Makes the &#39;keys&#39; arg perform a partial match. Defaults to false. | 
 **stale** | **bool**| For internal use only, please do not use this. | 
 **type_info** | **bool**| The type ID is used by internal services. For internal use only, please do not use this. | 
 **memory_only** | **bool**| Only use statistics sources that reside in memory (faster, but with less retention). | 
 **key** | [**Vec&lt;String&gt;**](String.md)| One key name. Can be used more than one time to query multiple keys. Also works with &#39;keys&#39; arguments. | 
 **degraded** | **bool**| If true, try to continue even if some stats are unavailable. In this case, errors will be present in the per-key returned data. | 
 **show_nodes** | **bool**| Shows the logical node number or LNN in addition to the devid. | 
 **resolution** | **i32**| Synonymous with &#39;interval&#39;, if supplied supersedes interval. | 
 **nodes** | [**Vec&lt;String&gt;**](String.md)| Specify node(s) for which statistics should be reported. One or more comma separated &lt;integer(s)&gt; specifying which node(s) to query, or \&quot;all\&quot;. Specifying more than one node may have unspecified results for keys that begin with \&quot;cluster\&quot;. | 

### Return type

[**::models::StatisticsHistory**](StatisticsHistory.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_statistics_key**
>crate::models::StatisticsKeys get_statistics_key(ctx, statistics_key_id)


List key meta-data.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **statistics_key_id** | **String**| List key meta-data. | 

### Return type

[**::models::StatisticsKeys**](StatisticsKeys.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_statistics_keys**
>crate::models::StatisticsKeysExtended get_statistics_keys(ctx, optional)


List meta-data for matching keys.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **bool**| Only count matching keys, do not return meta-data. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **queryable** | **bool**| Only list keys that can/cannot be queries. Default is true. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::StatisticsKeysExtended**](StatisticsKeysExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_statistics_operations**
>crate::models::StatisticsOperations get_statistics_operations(ctx, optional)


Retrieve operations list.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **protocols** | [**Vec&lt;String&gt;**](String.md)| A comma separated list. Only report operations for specified protocol(s). Default is all.  | 

### Return type

[**::models::StatisticsOperations**](StatisticsOperations.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_statistics_protocols**
>crate::models::StatisticsProtocols get_statistics_protocols(ctx, optional)


Retrieve protocol list.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**| Specifies whether internal, external, or all protocols should be returned. | 

### Return type

[**::models::StatisticsProtocols**](StatisticsProtocols.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_summary_client**
>crate::models::SummaryClient get_summary_client(ctx, optional)




### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Sort data by the specified comma-separated field(s). (num_operations | operation_rate | in_max | in_min | in | in_avg | out_max | out_min | out | out_avg | time_max | time_min | time_avg | node | protocol | class | user_id | user_name | local_addr | local_name | remote_addr | remote_name) Prepend &#39;asc:&#39; or &#39;desc:&#39; to a field to change the sort direction.  | 
 **totalby** | **String**| A comma separated list specifying what should be unique. (node | protocol | class | local_addr | local_name | remote_addr | remote_name | user_id | user_name | devid). Aggregation is performed over all the fields not specified in the list. | 
 **user_names** | **String**| A comma separated list. Only report statistics for operations requested by users with resolved names enumerated.  | 
 **remote_addresses** | **String**| A comma separated list. Only report statistics for operations requested by the remote clients with dotted-quad IP addresses enumerated.  | 
 **numeric** | **bool**| Do not resolve hostnames and usernames to their human readable form(s). Default is false.  | 
 **local_names** | **String**| A comma separated list. Only report statistics for operations handled by the local hosts with resolved names enumerated.  | 
 **user_ids** | **String**| A comma separated list. Only report statistics for operations requested by users with numeric UIDs enumerated.  | 
 **classes** | **String**| A comma separated list. Default is all. (other | write | read | namespace_read | namespace_write) | 
 **timeout** | **i32**| Timeout remote commands after NUM seconds, where NUM is the integer passed to the argument. | 
 **local_addresses** | **String**| A comma separated list. Only report statistics for operations handled by the local hosts with dotted-quad IP addresses enumerated.  | 
 **degraded** | **bool**| Continue to report if some nodes do not respond. | 
 **remote_names** | **String**| A comma separated list. Only report statistics for operations requested by the remote clients with resolved names enumerated.  | 
 **nodes** | **String**| A comma separated list. Specify node(s) for which statistics should be reported. Default is all. Zero (0) should be passed in as the sole argument to indicate local. | 
 **protocols** | **String**| A comma separated list. Default is all. (nfs3 | smb1 | nlm | ftp | http | siq | smb2 | nfs4 | papi | jobd | irp | lsass_in | lsass_out | hdfs | internal | external) | 

### Return type

[**::models::SummaryClient**](SummaryClient.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_summary_drive**
>crate::models::SummaryDrive get_summary_drive(ctx, optional)




### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Sort data by the specified comma-separated field(s). (drive_id | type | xfers_in | bytes_in | xfer_size_in | xfers_out | bytes_out | xfer_size_out | access_latency | access_slow | iosched_latency | iosched_queue | busy | used_bytes_percent | used_inodes). Prepend &#39;asc:&#39; or &#39;desc:&#39; to a field to change the sort direction.  | 
 **degraded** | **bool**| Continue to report if some nodes do not respond. | 
 **_type** | **String**| Specify drive type(s) for which statistics should be reported. | 
 **nodes** | **String**| A comma separated list. Specify node(s) for which statistics should be reported. Default is all. Zero (0) should be passed in as the sole argument to indicate local. | 
 **timeout** | **i32**| Timeout remote commands after NUM seconds, where NUM is the integer passed to the argument. | 

### Return type

[**::models::SummaryDrive**](SummaryDrive.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_summary_heat**
>crate::models::SummaryHeat get_summary_heat(ctx, optional)


File heat map, i.e. rate of file operations, and the type of operation listed by path/lin(s).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Sort data by the specified comma-separated field(s). (operation_rate | node | event_name | class_name | lin | path). Prepend &#39;asc:&#39; or &#39;desc:&#39; to a field to change the sort direction.  | 
 **convertlin** | **bool**| Convert lin to hex. Default is true.  | 
 **totalby** | **String**| A comma separated list specifying what should be unique. (node | event_name | event_class | operation_rate | path | lin). Aggregation is performed over all the fields not specified in the list. | 
 **pathdepth** | **i32**| Squash paths to this directory depth. Defaults to none, ie. the paths are not limited (Subject to the system limits.) | 
 **numeric** | **bool**| Do not resolve LINs into filenames. Default is false.  | 
 **events** | **String**| A comma separated list. Default is all. Only report specified event types(s). (blocked | contended | deadlocked | getattr | link | lock | lookup | read | rename | setattr | unlink | write). | 
 **maxpath** | **i32**| Maximum bytes allocated for looking up a path. An ASCII character is 1 byte (It may be more for other types of encoding). The default is 1024 bytes. Zero (0) means unlimited (Subject to the system limits.) | 
 **classes** | **String**| A comma separated list. Default is all. (other | write | read | namespace_read | namespace_write). | 
 **timeout** | **i32**| Timeout remote commands after NUM seconds, where NUM is the integer passed to the argument. | 
 **nodes** | **String**| A comma separated list. Specify node(s) for which statistics should be reported. Default is all. Zero (0) should be passed in as the sole argument to indicate local. | 
 **degraded** | **bool**| Continue to report if some nodes do not respond. | 

### Return type

[**::models::SummaryHeat**](SummaryHeat.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_summary_protocol**
>crate::models::SummaryProtocol get_summary_protocol(ctx, optional)




### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **operations** | **String**| Specify operation(s) for which statistics should be reported (See the cli command: &#39;isi statistics list operations&#39;, for a total list). Default is all.  | 
 **sort** | **String**| Sort data by the specified comma-separated field(s). (time | operation_count | operation_rate | in_max | in_min | in | in_avg | in_standard_dev | out_max | out_min | out | out_avg | out_standard_dev | time_max | time_min | time_avg | time_standard_dev | node | protocol | class | operation). Prepend &#39;asc:&#39; or &#39;desc:&#39; to a field to change the sort direction.  | 
 **totalby** | **String**| A comma separated list specifying what should be unique. (node | protocol | class | operation). Aggregation is performed over all the fields not specified in the list. | 
 **zero** | **bool**| Show table entries with no values. | 
 **classes** | **String**| A comma separated list. Default is all. (other | write | read | create | delete | namespace_read | namespace_write | file_state | session_state). | 
 **timeout** | **i32**| Timeout remote commands after NUM seconds, where NUM is the integer passed to the argument. | 
 **degraded** | **bool**| Continue to report if some nodes do not respond. | 
 **nodes** | **String**| A comma separated list. Specify node(s) for which statistics should be reported. Default is all. Zero (0) should be passed in as the sole argument to indicate local. | 
 **protocols** | **String**| A comma separated list. Default is all external protocols. (nfs3 | smb1 | nlm | ftp | http | siq | smb2 | nfs4 | papi | jobd | irp | lsass_in | lsass_out | hdfs | all | internal | external) | 

### Return type

[**::models::SummaryProtocol**](SummaryProtocol.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_summary_protocol_stats**
>crate::models::SummaryProtocolStats get_summary_protocol_stats(ctx, optional)




### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **degraded** | **bool**| Continue to report if some nodes do not respond. | 
 **protocol** | **String**| A single protocol. Default is nfs3. (nfs3 | smb1 | nlm | ftp | http | siq | smb2 | nfs4 | papi | jobd | irp | lsass_in | lsass_out | hdfs) | 
 **nodes** | **String**| A comma separated list. Specify node(s) for which statistics should be reported. Default is all. Zero (0) should be passed in as the sole argument to indicate local. | 
 **timeout** | **i32**| Timeout remote commands after NUM seconds, where NUM is the integer passed to the argument. | 

### Return type

[**::models::SummaryProtocolStats**](SummaryProtocolStats.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_summary_system**
>crate::models::SummarySystem get_summary_system(ctx, optional)




### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Sort data by the specified comma-separated field(s). (time | node | cpu | smb | ftp | http | nfs | hdfs | total | net_in | net_out | disk_in). Prepend &#39;asc:&#39; or &#39;desc:&#39; to a field to change the sort direction.  | 
 **oprates** | **bool**| Display protocol operation rate statistics rather than the default throughput statistics. | 
 **degraded** | **bool**| Continue to report if some nodes do not respond. | 
 **nodes** | **String**| A comma separated list. Specify node(s) for which statistics should be reported. Default is all. Zero (0) should be passed in as the sole argument to indicate local. | 
 **timeout** | **i32**| Timeout remote commands after NUM seconds, where NUM is the integer passed to the argument. | 

### Return type

[**::models::SummarySystem**](SummarySystem.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_summary_workload**
>crate::models::SummaryWorkload get_summary_workload(ctx, optional)




### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Sort data by the specified comma-separated field(s). (node | system_name | job_type | cpu | reads | writes | l2 | l3). Prepend &#39;asc:&#39; or &#39;desc:&#39; to a field to change the sort direction.  | 
 **job_types** | [**Vec&lt;String&gt;**](String.md)| A comma separated list. Only report statistics for job(s) specified by type, if configured.  | 
 **totalby** | **String**| A comma separated list specifying what should be unique. (node | system_name | job_type). Aggregation is performed over all the fields not specified in the list. | 
 **timeout** | **i32**| Timeout remote commands after NUM seconds, where NUM is the integer passed to the argument. | 
 **degraded** | **bool**| Continue to report if some nodes do not respond. | 
 **nodes** | **String**| A comma separated list. Specify node(s) for which statistics should be reported. Default is all. Zero (0) should be passed in as the sole argument to indicate local. | 
 **system_names** | **String**| A comma separated list. Only report statistics for workloads specified by system names. | 

### Return type

[**::models::SummaryWorkload**](SummaryWorkload.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

