# \EventApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_event_alert_condition**](EventApi.md#create_event_alert_condition) | **Post** /platform/4/event/alert-conditions | 
[**create_event_channel**](EventApi.md#create_event_channel) | **Post** /platform/3/event/channels | 
[**create_event_event**](EventApi.md#create_event_event) | **Post** /platform/3/event/events | 
[**delete_event_alert_condition**](EventApi.md#delete_event_alert_condition) | **Delete** /platform/4/event/alert-conditions/{EventAlertConditionId} | 
[**delete_event_alert_conditions**](EventApi.md#delete_event_alert_conditions) | **Delete** /platform/4/event/alert-conditions | 
[**delete_event_channel**](EventApi.md#delete_event_channel) | **Delete** /platform/3/event/channels/{EventChannelId} | 
[**get_event_alert_condition**](EventApi.md#get_event_alert_condition) | **Get** /platform/4/event/alert-conditions/{EventAlertConditionId} | 
[**get_event_categories**](EventApi.md#get_event_categories) | **Get** /platform/3/event/categories | 
[**get_event_category**](EventApi.md#get_event_category) | **Get** /platform/3/event/categories/{EventCategoryId} | 
[**get_event_channel**](EventApi.md#get_event_channel) | **Get** /platform/3/event/channels/{EventChannelId} | 
[**get_event_eventgroup_definition**](EventApi.md#get_event_eventgroup_definition) | **Get** /platform/4/event/eventgroup-definitions/{EventEventgroupDefinitionId} | 
[**get_event_eventgroup_definitions**](EventApi.md#get_event_eventgroup_definitions) | **Get** /platform/4/event/eventgroup-definitions | 
[**get_event_eventgroup_occurrence**](EventApi.md#get_event_eventgroup_occurrence) | **Get** /platform/3/event/eventgroup-occurrences/{EventEventgroupOccurrenceId} | 
[**get_event_eventgroup_occurrences**](EventApi.md#get_event_eventgroup_occurrences) | **Get** /platform/3/event/eventgroup-occurrences | 
[**get_event_eventlist**](EventApi.md#get_event_eventlist) | **Get** /platform/3/event/eventlists/{EventEventlistId} | 
[**get_event_eventlists**](EventApi.md#get_event_eventlists) | **Get** /platform/3/event/eventlists | 
[**get_event_settings**](EventApi.md#get_event_settings) | **Get** /platform/3/event/settings | 
[**list_event_alert_conditions**](EventApi.md#list_event_alert_conditions) | **Get** /platform/4/event/alert-conditions | 
[**list_event_channels**](EventApi.md#list_event_channels) | **Get** /platform/3/event/channels | 
[**update_event_alert_condition**](EventApi.md#update_event_alert_condition) | **Put** /platform/4/event/alert-conditions/{EventAlertConditionId} | 
[**update_event_channel**](EventApi.md#update_event_channel) | **Put** /platform/3/event/channels/{EventChannelId} | 
[**update_event_eventgroup_occurrence**](EventApi.md#update_event_eventgroup_occurrence) | **Put** /platform/3/event/eventgroup-occurrences/{EventEventgroupOccurrenceId} | 
[**update_event_eventgroup_occurrences**](EventApi.md#update_event_eventgroup_occurrences) | **Put** /platform/3/event/eventgroup-occurrences | 
[**update_event_settings**](EventApi.md#update_event_settings) | **Put** /platform/3/event/settings | 


# **create_event_alert_condition**
> ::models::CreateResponse create_event_alert_condition(ctx, event_alert_condition)


Create a new alert condition.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_alert_condition** | [**EventAlertConditionCreateParams**](EventAlertConditionCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_event_channel**
> ::models::CreateResponse create_event_channel(ctx, event_channel)


Create a new channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_channel** | [**EventChannelCreateParams**](EventChannelCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_event_event**
> ::models::CreateQuotaReportResponse create_event_event(ctx, event_event)


Create a test event.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_event** | [**EventEvent**](EventEvent.md)|  | 

### Return type

[**::models::CreateQuotaReportResponse**](CreateQuotaReportResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_event_alert_condition**
> delete_event_alert_condition(ctx, event_alert_condition_id)


Delete the alert-condition.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_alert_condition_id** | **String**| Delete the alert-condition. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_event_alert_conditions**
> delete_event_alert_conditions(ctx, optional)


Bulk delete of alert conditions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **channel** | **String**| Delete only conditions for this channel | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_event_channel**
> delete_event_channel(ctx, event_channel_id)


Delete the channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_channel_id** | **String**| Delete the channel. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_alert_condition**
> ::models::EventAlertConditions get_event_alert_condition(ctx, event_alert_condition_id)


Retrieve the alert-condition.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_alert_condition_id** | **String**| Retrieve the alert-condition. | 

### Return type

[**::models::EventAlertConditions**](EventAlertConditions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_categories**
> ::models::EventCategoriesExtended get_event_categories(ctx, optional)


List all eventgroup categories.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::EventCategoriesExtended**](EventCategoriesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_category**
> ::models::EventCategories get_event_category(ctx, event_category_id)


Retrieve the eventgroup category.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_category_id** | **String**| Retrieve the eventgroup category. | 

### Return type

[**::models::EventCategories**](EventCategories.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_channel**
> ::models::EventChannels get_event_channel(ctx, event_channel_id)


Retrieve the channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_channel_id** | **String**| Retrieve the channel. | 

### Return type

[**::models::EventChannels**](EventChannels.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_eventgroup_definition**
> ::models::EventEventgroupDefinitions get_event_eventgroup_definition(ctx, event_eventgroup_definition_id)


Retrieve the eventgroup definition.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_eventgroup_definition_id** | **String**| Retrieve the eventgroup definition. | 

### Return type

[**::models::EventEventgroupDefinitions**](EventEventgroupDefinitions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_eventgroup_definitions**
> ::models::EventEventgroupDefinitionsExtended get_event_eventgroup_definitions(ctx, optional)


List all eventgroup definitions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **category** | **i32**| Return eventgroups in the specified category | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::EventEventgroupDefinitionsExtended**](EventEventgroupDefinitionsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_eventgroup_occurrence**
> ::models::EventEventgroupOccurrences get_event_eventgroup_occurrence(ctx, event_eventgroup_occurrence_id)


Retrieve individual eventgroup occurrence.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_eventgroup_occurrence_id** | **String**| Retrieve individual eventgroup occurrence. | 

### Return type

[**::models::EventEventgroupOccurrences**](EventEventgroupOccurrences.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_eventgroup_occurrences**
> ::models::EventEventgroupOccurrencesExtended get_event_eventgroup_occurrences(ctx, optional)


List all eventgroup occurrences.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **resolved** | **bool**| Filter for resolved eventgroups | 
 **sort** | **String**| The field that will be used for sorting. | 
 **begin** | **i32**| events that are in progress after this time | 
 **end** | **i32**| events that were in progress before this time | 
 **event_count** | **i32**| events for which event_count &gt; this | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **ignore** | **bool**| Filter for ignored eventgroups | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resolver** | **String**| Filter for eventgroup resolver | 
 **cause** | **String**| Filter for cause | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::EventEventgroupOccurrencesExtended**](EventEventgroupOccurrencesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_eventlist**
> ::models::EventEventlists get_event_eventlist(ctx, event_eventlist_id)


Retrieve the list of events for a eventgroup occureence.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_eventlist_id** | **String**| Retrieve the list of events for a eventgroup occureence. | 

### Return type

[**::models::EventEventlists**](EventEventlists.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_eventlists**
> ::models::EventEventlistsExtended get_event_eventlists(ctx, optional)


List all event occurrences grouped by eventgroup occurrence.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **event_instance** | **String**| Return only this event occurrence | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::EventEventlistsExtended**](EventEventlistsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event_settings**
> ::models::EventSettings get_event_settings(ctx, )


Retrieve the settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::EventSettings**](EventSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_event_alert_conditions**
> ::models::EventAlertConditionsExtended list_event_alert_conditions(ctx, optional)


List all alert conditions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **channels** | **String**| Return only conditions for the specified channel: | 
 **sort** | **String**| The field that will be used for sorting. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::EventAlertConditionsExtended**](EventAlertConditionsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_event_channels**
> ::models::EventChannelsExtended list_event_channels(ctx, optional)


List all channels.

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

[**::models::EventChannelsExtended**](EventChannelsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_event_alert_condition**
> update_event_alert_condition(ctx, event_alert_condition, event_alert_condition_id)


Modify the alert-condition

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_alert_condition** | [**EventAlertCondition**](EventAlertCondition.md)|  | 
  **event_alert_condition_id** | **String**| Modify the alert-condition | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_event_channel**
> update_event_channel(ctx, event_channel, event_channel_id)


Modify the channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_channel** | [**EventChannel**](EventChannel.md)|  | 
  **event_channel_id** | **String**| Modify the channel. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_event_eventgroup_occurrence**
> update_event_eventgroup_occurrence(ctx, event_eventgroup_occurrence, event_eventgroup_occurrence_id)


modify eventgroup occurrence.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_eventgroup_occurrence** | [**EventEventgroupOccurrence**](EventEventgroupOccurrence.md)|  | 
  **event_eventgroup_occurrence_id** | **String**| modify eventgroup occurrence. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_event_eventgroup_occurrences**
> update_event_eventgroup_occurrences(ctx, event_eventgroup_occurrences)


Modify all eventgroup occurrences, resolve or ignore all

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_eventgroup_occurrences** | [**EventEventgroupOccurrence**](EventEventgroupOccurrence.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_event_settings**
> update_event_settings(ctx, event_settings)


Update settings

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_settings** | [**EventSettings**](EventSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

