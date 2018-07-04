# SyncJobPolicyFileMatchingPatternOrCriteriaItemAndCriteriaItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attribute_exists** | **bool** | For \&quot;custom_attribute\&quot; type criteria.  The file will match as long as the attribute named by \&quot;field\&quot; exists.  Default is true. | [optional] [default to null]
**case_sensitive** | **bool** | If true, the value comparison will be case sensitive.  Default is true. | [optional] [default to null]
**field** | **String** | The name of the file attribute to match on (only required if this is a custom_attribute type criterion).  Default is an empty string \&quot;\&quot;. | [optional] [default to null]
**operator** | **String** | How to compare the specified attribute of each file to the specified value. | [optional] [default to null]
**_type** | **String** | The type of this criterion, that is, which file attribute to match on. | [default to null]
**value** | **String** | The value to compare the specified attribute of each file to. | [optional] [default to null]
**whole_word** | **bool** | If true, the attribute must match the entire word.  Default is true. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


