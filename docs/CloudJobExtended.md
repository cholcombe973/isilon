# CloudJobExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_time** | **i32** | The time at which the job was completed (if applicable) | [optional] [default to null]
**create_time** | **i32** | The time at which the job was created | [optional] [default to null]
**description** | **String** | A brief description of the job contents | [optional] [default to null]
**effective_state** | **String** | The effective state of the job (e.g,. the combination of operation_state and job_state) | [optional] [default to null]
**files** | [***::models::CloudJobFiles**](CloudJobFiles.md) | The files and filters addressed by this job | [optional] [default to null]
**id** | **i32** | The job&#39;s ID | [optional] [default to null]
**job_engine_job** | [***::models::CloudJobJobEngineJob**](CloudJobJobEngineJob.md) | Information about the related job engine job if there is one | [optional] [default to null]
**job_state** | **String** | The current state of the job | [optional] [default to null]
**operation_state** | **String** | The current state of the operation associated with the job | [optional] [default to null]
**state_change_time** | **i32** | The last time at which the job state changed | [optional] [default to null]
**_type** | **String** | The type of cloud action to be performed by this job. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


