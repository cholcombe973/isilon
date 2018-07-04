# CloudPoolCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | **Vec<String>** | A list of valid names for the accounts in this pool.  There is currently only one account allowed per pool. | [default to null]
**birth_cluster_id** | **String** | The guid of the cluster where this pool was created | [optional] [default to null]
**description** | **String** | A brief description of this pool | [optional] [default to null]
**name** | **String** | A unique name for this pool | [default to null]
**vendor** | **String** | A string identifier of the cloud services vendor | [optional] [default to null]
**_type** | **String** | The type of cloud protocol required.  E.g., \&quot;isilon\&quot; for EMC Isilon, \&quot;ecs\&quot; for EMC ECS Appliance, \&quot;virtustream\&quot; for Virtustream Storage Cloud, \&quot;azure\&quot; for Microsoft Azure and \&quot;s3\&quot; for Amazon S3 | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


