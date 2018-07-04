# WormDomain

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autocommit_offset** | **i32** | Specifies the autocommit time period for the domain in seconds.  After a file is in the domain without being modified for the specified time period, the file is automatically committed. If this parameter is set to null, there is no autocommit time, and files must be committed manually. | [optional] [default to null]
**default_retention** | **String** |  | [optional] [default to null]
**max_retention** | **String** |  | [optional] [default to null]
**min_retention** | **String** |  | [optional] [default to null]
**override_date** | **i32** | Specifies the override retention date for the domain. If this date is later than the retention date for any committed file, the file will remain protected until the override retention date. | [optional] [default to null]
**privileged_delete** | **String** | When this value is set to &#39;on&#39;, files in this domain can be deleted through the privileged delete feature. If this value is set to &#39;disabled&#39;, privileged file deletes are permanently disabled and cannot be turned on again. | [optional] [default to null]
**_type** | **String** | Specifies whether the domain is an enterprise domain or a compliance domain. Compliance domains can not be created on enterprise clusters. Enterprise and compliance domains can be created on compliance clusters. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


