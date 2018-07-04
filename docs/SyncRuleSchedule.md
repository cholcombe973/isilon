# SyncRuleSchedule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**begin** | **String** | Start time (inclusive) for this schedule, during its specified days.  Format is \&quot;hh:mm\&quot; (24h format hour, and minute).  A null value indicates the beginning of the day (\&quot;00:00\&quot;). | [optional] [default to null]
**end** | **String** | End time (inclusive) for this schedule, during its specified days.  Format is \&quot;hh:mm\&quot; (three-letter weekday name abbreviation, 24h format hour, and minute).  A null value indicates the end of the day (\&quot;23:59\&quot;). | [optional] [default to null]
**friday** | **bool** | If true, this rule is in effect on Friday.  If false, or unspecified, it is not. | [optional] [default to null]
**monday** | **bool** | If true, this rule is in effect on Monday.  If false, or unspecified, it is not. | [optional] [default to null]
**saturday** | **bool** | If true, this rule is in effect on Saturday.  If false, or unspecified, it is not. | [optional] [default to null]
**sunday** | **bool** | If true, this rule is in effect on Sunday.  If false, or unspecified, it is not. | [optional] [default to null]
**thursday** | **bool** | If true, this rule is in effect on Thursday.  If false, or unspecified, it is not. | [optional] [default to null]
**tuesday** | **bool** | If true, this rule is in effect on Tuesday.  If false, or unspecified, it is not. | [optional] [default to null]
**wednesday** | **bool** | If true, this rule is in effect on Wednesday.  If false, or unspecified, it is not. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


