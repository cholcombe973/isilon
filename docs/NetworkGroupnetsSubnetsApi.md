# \NetworkGroupnetsSubnetsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pools_pool_rebalance_ip**](NetworkGroupnetsSubnetsApi.md#create_pools_pool_rebalance_ip) | **Post** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/rebalance-ips | 
[**create_pools_pool_rule**](NetworkGroupnetsSubnetsApi.md#create_pools_pool_rule) | **Post** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/rules | 
[**create_pools_pool_sc_resume_node**](NetworkGroupnetsSubnetsApi.md#create_pools_pool_sc_resume_node) | **Post** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/sc-resume-nodes | 
[**create_pools_pool_sc_suspend_node**](NetworkGroupnetsSubnetsApi.md#create_pools_pool_sc_suspend_node) | **Post** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/sc-suspend-nodes | 
[**delete_pools_pool_rule**](NetworkGroupnetsSubnetsApi.md#delete_pools_pool_rule) | **Delete** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/rules/{PoolsPoolRuleId} | 
[**get_pools_pool_interfaces**](NetworkGroupnetsSubnetsApi.md#get_pools_pool_interfaces) | **Get** /platform/4/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/interfaces | 
[**get_pools_pool_rule**](NetworkGroupnetsSubnetsApi.md#get_pools_pool_rule) | **Get** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/rules/{PoolsPoolRuleId} | 
[**list_pools_pool_rules**](NetworkGroupnetsSubnetsApi.md#list_pools_pool_rules) | **Get** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/rules | 
[**update_pools_pool_rule**](NetworkGroupnetsSubnetsApi.md#update_pools_pool_rule) | **Put** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{Pool}/rules/{PoolsPoolRuleId} | 


# **create_pools_pool_rebalance_ip**
> ::models::Empty create_pools_pool_rebalance_ip(ctx, pools_pool_rebalance_ip, groupnet, subnet, pool)


Rebalance IP addresses in specified pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pools_pool_rebalance_ip** | [**Empty**](Empty.md)|  | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_pools_pool_rule**
> ::models::CreateResponse create_pools_pool_rule(ctx, pools_pool_rule, groupnet, subnet, pool)


Create a new rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pools_pool_rule** | [**PoolsPoolRuleCreateParams**](PoolsPoolRuleCreateParams.md)|  | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_pools_pool_sc_resume_node**
> ::models::Empty create_pools_pool_sc_resume_node(ctx, pools_pool_sc_resume_node, groupnet, subnet, pool)


Resume suspended nodes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pools_pool_sc_resume_node** | [**PoolsPoolScResumeNode**](PoolsPoolScResumeNode.md)|  | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_pools_pool_sc_suspend_node**
> ::models::Empty create_pools_pool_sc_suspend_node(ctx, pools_pool_sc_suspend_node, groupnet, subnet, pool)


Suspend nodes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pools_pool_sc_suspend_node** | [**PoolsPoolScResumeNode**](PoolsPoolScResumeNode.md)|  | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_pools_pool_rule**
> delete_pools_pool_rule(ctx, pools_pool_rule_id, groupnet, subnet, pool)


Delete a network rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pools_pool_rule_id** | **String**| Delete a network rule. | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pools_pool_interfaces**
> ::models::PoolsPoolInterfaces get_pools_pool_interfaces(ctx, groupnet, subnet, pool, optional)


Get a list of interfaces.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet** | **String**|  | 
 **subnet** | **String**|  | 
 **pool** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **lnns** | **String**| Get a list of interfaces for the specified lnn. | 

### Return type

[**::models::PoolsPoolInterfaces**](PoolsPoolInterfaces.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pools_pool_rule**
> ::models::PoolsPoolRules get_pools_pool_rule(ctx, pools_pool_rule_id, groupnet, subnet, pool)


View a single network rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pools_pool_rule_id** | **String**| View a single network rule. | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 

### Return type

[**::models::PoolsPoolRules**](PoolsPoolRules.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_pools_pool_rules**
> ::models::PoolsPoolRulesExtended list_pools_pool_rules(ctx, groupnet, subnet, pool, optional)


Get a list of network rules.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet** | **String**|  | 
 **subnet** | **String**|  | 
 **pool** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::PoolsPoolRulesExtended**](PoolsPoolRulesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_pools_pool_rule**
> update_pools_pool_rule(ctx, pools_pool_rule, pools_pool_rule_id, groupnet, subnet, pool)


Modify a network rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pools_pool_rule** | [**PoolsPoolRule**](PoolsPoolRule.md)|  | 
  **pools_pool_rule_id** | **String**| Modify a network rule. | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
  **pool** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

