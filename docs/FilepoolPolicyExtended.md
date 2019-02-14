# FilepoolPolicyExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | [**Vec <crate::models::FilepoolPolicyAction>**](FilepoolPolicyAction.md) | A list of actions to be taken for matching files | [optional] [default to null]
**apply_order** | **i32** | The order in which this policy should be applied (relative to other policies) | [optional] [default to null]
**birth_cluster_id** | **String** | The guid assigned to the cluster on which the account was created | [optional] [default to null]
**description** | **String** | A description for this policy | [optional] [default to null]
**file_matching_pattern** | [***::models::FilepoolPolicyFileMatchingPattern**](FilepoolPolicyFileMatchingPattern.md) | The file matching rules for this policy | [optional] [default to null]
**id** | **i32** | A unique identifier for this policy | [optional] [default to null]
**name** | **String** | A unique name for this policy | [optional] [default to null]
**state** | **String** | Indicates whether this policy is in a good state (\&quot;OK\&quot;) or disabled (\&quot;disabled\&quot;) | [optional] [default to null]
**state_details** | **String** | Gives further information to describe the state of this policy | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


