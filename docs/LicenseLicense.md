# LicenseLicense

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**days_since_expiry** | **i32** | Number of days since a license expired. | [optional] [default to null]
**days_to_expiry** | **i32** | Number of days before a license expires. | [optional] [default to null]
**expiration** | **String** | Date of license expiry. Format is YYYY-MM-DD. It is not included if there is no expiration. Feature is considered expired at end of this day. The cluster time is used to determine expiry. | [optional] [default to null]
**expired_alert** | **bool** | True when we are generating an alert that this feature has expired. | [default to null]
**expiring_alert** | **bool** | True when we are generating an alert that this feature is expiring. | [default to null]
**id** | **String** | Name of the licensed feature. | [default to null]
**name** | **String** | Name of the licensed feature. | [default to null]
**status** | **String** | Current status of the license. | [default to null]
**tiers** | [**Vec <crate::models::LicenseLicenseTier>**](LicenseLicenseTier.md) | Tiered License details. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


