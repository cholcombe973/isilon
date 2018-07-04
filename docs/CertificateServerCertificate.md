# CertificateServerCertificate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default** | **bool** | Boolean identifying if a certificate is the default certificate.The default certificate is used as the fallback when no other certificates match a TLS enabled service&#39;s particular criteria. There must always be a configured default certificate. | [default to null]
**description** | **String** | Description field associated with a certificate provided for administrative convenience. | [default to null]
**dnsnames** | **Vec<String>** | A list of DNS names/patterns for which this certificate is valid. This list is extracted from the certificates CN (Common Name) and subjectAtlName extension fields. | [default to null]
**expired** | **bool** | True if the certificate has expired and is no longer valid. | [default to null]
**fingerprints** | [**Vec<::models::CertificateServerCertificateFingerprint>**](CertificateServerCertificateFingerprint.md) | A list of zero or more certificate fingerprints which can be used for certificate identification. | [default to null]
**id** | **String** | Unique server certificate identifier. | [default to null]
**issuer** | **String** | Certificate issuer field extracted from the certificate. | [default to null]
**name** | **String** | Subject common name extracted from the certificate. | [default to null]
**not_after** | **i32** | Certificate notAfter field extracted from the certificate encoded as a UNIX epoch timestamp.  The certificate is not valid after this timestamp. | [default to null]
**not_before** | **i32** | Certificate notBefore field extracted from the certificate encoded as a UNIX epoch timestamp.  The certificate is not valid before this timestamp. | [default to null]
**subject** | **String** | Certificate subject field extracted from the certificate. | [default to null]
**valid** | **bool** | True if the certificate is valid (ie: not_before &lt;&#x3D; now &lt;&#x3D; not_after). | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


