# EventChannel

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_nodes** | **Vec<i32>** | Nodes (LNNs) that can be masters for this channel | [optional] [default to null]
**enabled** | **bool** | Channel is to be used or not | [optional] [default to null]
**excluded_nodes** | **Vec<i32>** | Nodes (LNNs) that can NOT be the masters for this channel | [optional] [default to null]
**parameters** | [***::models::EventChannelParameters**](EventChannelParameters.md) | Parameters to be used for an smtp channel | [optional] [default to null]
**system** | **bool** | Channel is a pre-defined system channel | [optional] [default to null]
**_type** | **String** | The mechanism used by the channel | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


