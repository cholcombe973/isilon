# NetworkDnscacheSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_entry_limit** | **i32** | DNS cache entry limit | [default to null]
**cluster_timeout** | **i32** | Timeout value for calls made to other nodes in the cluster | [default to null]
**dns_timeout** | **i32** | Timeout value for calls made to the dns resolvers | [default to null]
**eager_refresh** | **i32** | Lead time to refresh cache entries nearing expiration | [default to null]
**testping_delta** | **i32** | Deltas for checking cbind cluster health | [default to null]
**ttl_max_noerror** | **i32** | Upper bound on ttl for cache hits | [default to null]
**ttl_max_nxdomain** | **i32** | Upper bound on ttl for nxdomain | [default to null]
**ttl_max_other** | **i32** | Upper bound on ttl for non-nxdomain failures | [default to null]
**ttl_max_servfail** | **i32** | Upper bound on ttl for server failures | [default to null]
**ttl_min_noerror** | **i32** | Lower bound on ttl for cache hits | [default to null]
**ttl_min_nxdomain** | **i32** | Lower bound on ttl for nxdomain | [default to null]
**ttl_min_other** | **i32** | Lower bound on ttl for non-nxdomain failures | [default to null]
**ttl_min_servfail** | **i32** | Lower bound on ttl for server failures | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


