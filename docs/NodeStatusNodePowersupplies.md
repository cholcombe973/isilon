# NodeStatusNodePowersupplies

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **i32** | Count of how many power supplies are supported. | [optional] [default to null]
**failures** | **i32** | Count of how many power supplies have failed. | [optional] [default to null]
**has_cff** | **bool** | Does this node have a CFF power supply. | [optional] [default to null]
**status** | **String** | A descriptive status string for this node&#39;s power supplies. | [optional] [default to null]
**supplies** | [**Vec<::models::NodeStatusNodePowersuppliesSupply>**](NodeStatusNodePowersuppliesSupply.md) | List of this node&#39;s power supplies. | [optional] [default to null]
**supports_cff** | **bool** | Does this node support CFF power supplies. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


