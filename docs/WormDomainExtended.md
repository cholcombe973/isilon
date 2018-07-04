# WormDomainExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autocommit_offset** | **i32** | Specifies the autocommit time period for the domain in seconds.  After a file is in the domain without being modified for the specified time period, the file is automatically committed. If this parameter is set to null, there is no autocommit time, and files must be committed manually. | [optional] [default to null]
**default_retention** | **String** |  | [optional] [default to null]
**max_retention** | **String** |  | [optional] [default to null]
**min_retention** | **String** |  | [optional] [default to null]
**override_date** | **i32** | Specifies the override retention date for the domain. If this date is later than the retention date for any committed file, the file will remain protected until the override retention date. | [optional] [default to null]
**privileged_delete** | **String** | When this value is set to &#39;on&#39;, files in this domain can be deleted through the privileged delete feature. If this value is set to &#39;disabled&#39;, privileged file deletes are permanently disabled and cannot be turned on again. | [default to null]
**_type** | **String** | Specifies whether the domain is an enterprise domain or a compliance domain. Compliance domains can not be created on enterprise clusters. Enterprise and compliance domains can be created on compliance clusters. | [default to null]
**id** | **i32** | Specifies the system-assigned ID for the protection domain. | [default to null]
**incomplete** | **bool** | True if OneFS is still in the process of creating this domain and is unable to prevent files from being modified or deleted. If false, indicates that the domain is fully created and is able to prevent files from being modified or deleted. | [default to null]
**lin** | **i32** | Specifies the logical inode number (LIN) for the root of this domain. | [default to null]
**max_modifies** | **i32** | Specifies the maximum amount of time, in seconds, that a file in this domain will be protected. This setting will override the retention period of any file committed with a longer retention period. If this parameter is set to null, an infinite length retention period is set. | [default to null]
**path** | **String** | Specifies the root path of this domain. Files in this directory and all sub-directories will be protected. | [default to null]
**total_modifies** | **i32** | Specifies the number of times this domain has been modified and the number of times the attributes for the domain have changed. A SmartLock domain can be modified a fixed number of times as defined by the &#39;max_modifies&#39; parameter. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


