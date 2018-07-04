# JobJobCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_dup** | **bool** | Whether or not to queue the job if one of the same type is already running or queued. | [optional] [default to null]
**avscan_params** | [***::models::JobJobAvscanParams**](JobJobAvscanParams.md) |  | [optional] [default to null]
**changelistcreate_params** | [***::models::JobJobChangelistcreateParams**](JobJobChangelistcreateParams.md) |  | [optional] [default to null]
**domainmark_params** | [***::models::JobJobDomainmarkParams**](JobJobDomainmarkParams.md) |  | [optional] [default to null]
**paths** | **Vec<String>** | For jobs which take paths, the IFS paths to pass to the job. | [optional] [default to null]
**policy** | **String** | Impact policy of this job instance. | [optional] [default to null]
**prepair_params** | [***::models::JobJobPrepairParams**](JobJobPrepairParams.md) |  | [optional] [default to null]
**priority** | **i32** | Priority of this job instance; lower numbers preempt higher numbers. | [optional] [default to null]
**smartpoolstree_params** | [***::models::JobJobSmartpoolstreeParams**](JobJobSmartpoolstreeParams.md) |  | [optional] [default to null]
**snaprevert_params** | [***::models::JobJobSnaprevertParams**](JobJobSnaprevertParams.md) |  | [optional] [default to null]
**_type** | **String** | Job type to queue. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


