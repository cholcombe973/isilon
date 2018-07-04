# CloudAccountExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | (S3 only) The user id of the S3 account | [optional] [default to null]
**account_username** | **String** | The username required to authenticate against the cloud service | [optional] [default to null]
**birth_cluster_id** | **String** | The guid of the cluster where this account was created | [optional] [default to null]
**enabled** | **bool** | Whether this account is explicitly enabled or disabled by a user | [optional] [default to null]
**key** | **String** | A valid authentication key for connecting to the cloud | [optional] [default to null]
**name** | **String** | A unique name for this account | [optional] [default to null]
**proxy** | **String** | The id or name of a proxy to be used by this account | [optional] [default to null]
**skip_account_check** | **bool** | (Not recommended) Indicates whether to skip validation that the cloud account is still accessible | [optional] [default to null]
**skip_ssl_validation** | **bool** | Indicates whether to skip SSL certificate validation when connecting to the cloud | [optional] [default to null]
**storage_region** | **String** | (S3 only) An appropriate region for the S3 account.  For example, faster access times may be gained by referencing a nearby region | [optional] [default to null]
**telemetry_bucket** | **String** | (S3 only) The name of the bucket into which generated metrics reports are placed by the cloud service provider | [optional] [default to null]
**uri** | **String** | A valid URI pointing to the location of the cloud storage | [optional] [default to null]
**bucket** | **String** | The machine generated name of the account bucket to store data | [optional] [default to null]
**id** | **String** | A globally unique name for this account | [optional] [default to null]
**metadata_bucket** | **String** | The machine generated name of the account bucket to store metadata | [optional] [default to null]
**pool** | **String** | Name of the pool referencing this account.  Empty if none. | [optional] [default to null]
**state** | **String** | Indicates whether this account is in a good state (\&quot;OK\&quot;), disabled (\&quot;disabled\&quot;) or inaccessible via the network (\&quot;unreachable\&quot;) | [optional] [default to null]
**state_details** | **String** | Gives further information to describe the state of this account | [optional] [default to null]
**_type** | **String** | The type of cloud protocol required.  E.g., \&quot;isilon\&quot; for EMC Isilon, \&quot;ecs\&quot; for EMC ECS Appliance, \&quot;virtustream\&quot; for Virtustream Storage Cloud, \&quot;azure\&quot; for Microsoft Azure and \&quot;s3\&quot; for Amazon S3 | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


