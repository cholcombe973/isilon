# JobTypeExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | Whether the job type is enabled and able to run. | [default to null]
**policy** | **String** | Default impact policy of this job type. | [default to null]
**priority** | **i32** | Default priority of this job type; lower numbers preempt higher numbers. | [default to null]
**schedule** | **String** | The schedule on which this job type is queued, if any. | [optional] [default to null]
**allow_multiple_instances** | **bool** | Whether multiple instances of this job type may run simultaneously. | [default to null]
**description** | **String** | Brief description of the job type. | [default to null]
**exclusion_set** | **String** | The set(s) of mutually-exclusive job types to which this job belongs.  No job in this set may run with any other job in this set.  Obsolete; this value will always be an empty string, as exclusion sets are no longer a job type property. | [default to null]
**hidden** | **bool** | Whether this job type is normally visible in the UI. | [default to null]
**id** | **String** | Job type ID. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


