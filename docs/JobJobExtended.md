# JobJobExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**control_state** | **String** | State to which the job is transitioning; if control_state is identical to state, the job&#39;s state is stable. | [optional] [default to null]
**create_time** | **i32** | The time the job was queued, in seconds since the epoch. | [default to null]
**current_phase** | **i32** | The current phase of the job. | [optional] [default to null]
**description** | **String** | A text representation of the job. | [optional] [default to null]
**end_time** | **i32** | The time the job ended, in seconds since the Epoch. | [optional] [default to null]
**id** | **i32** | The ID of the job. | [default to null]
**impact** | **String** | The current impact of the job. | [default to null]
**participants** | **Vec<i32>** | The set of devids working on the job. | [optional] [default to null]
**paths** | **Vec<String>** | Paths for which the job was queued. | [optional] [default to null]
**policy** | **String** | Current impact policy of the job. | [default to null]
**priority** | **i32** | Current priority of the job; lower numbers preempt higher numbers. | [default to null]
**progress** | **String** | A text representation of the job&#39;s progress. | [optional] [default to null]
**retries_remaining** | **i32** | The number of retries remaining if the job fails. | [default to null]
**running_time** | **i32** | The number of seconds the job has executed. | [optional] [default to null]
**start_time** | **i32** | The time the job started, in seconds since the Epoch. | [optional] [default to null]
**state** | **String** | Current state of the job. | [default to null]
**total_phases** | **i32** | The total number of phases of the job type. | [default to null]
**_type** | **String** | The job type. | [default to null]
**waiting_on** | **i32** | The ID of a job for which this job is waiting. | [optional] [default to null]
**waiting_reason** | **String** | The reason the job is waiting. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


