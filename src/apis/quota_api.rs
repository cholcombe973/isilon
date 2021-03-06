/*
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::borrow::Borrow;
use std::rc::Rc;

use futures;
use futures::Future;
use hyper;

use super::{configuration, put, query, Error};

pub struct QuotaApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> QuotaApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> QuotaApiClient<C> {
        QuotaApiClient {
            configuration: configuration,
        }
    }
}

pub trait QuotaApi {
    fn create_quota_quota(
        &self,
        quota_quota: crate::models::QuotaQuotaCreateParams,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>>;
    fn create_quota_report(
        &self,
        quota_report: crate::models::Empty,
    ) -> Box<dyn Future<Item = crate::models::CreateQuotaReportResponse, Error = Error>>;
    fn create_settings_mapping(
        &self,
        settings_mapping: crate::models::SettingsMappingExtendedExtended,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>>;
    fn create_settings_notification(
        &self,
        settings_notification: crate::models::QuotaNotificationCreateParams,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>>;
    fn delete_quota_quota(&self, quota_quota_id: &str)
        -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_quota_quotas(
        &self,
        enforced: bool,
        include_snapshots: bool,
        zone: &str,
        recurse_path_children: bool,
        recurse_path_parents: bool,
        persona: &str,
        path: &str,
        _type: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_quota_report(
        &self,
        quota_report_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_settings_mapping(
        &self,
        settings_mapping_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_settings_mappings(&self) -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_settings_notification(
        &self,
        settings_notification_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_settings_notifications(&self) -> Box<dyn Future<Item = (), Error = Error>>;
    fn get_quota_license(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LicenseLicense, Error = Error>>;
    fn get_quota_quota(
        &self,
        quota_quota_id: &str,
        resolve_names: bool,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaQuotas, Error = Error>>;
    fn get_quota_quotas_summary(
        &self,
    ) -> Box<dyn Future<Item = crate::models::QuotaQuotasSummary, Error = Error>>;
    fn get_quota_report(
        &self,
        quota_report_id: &str,
        contents: bool,
    ) -> Box<dyn Future<Item = crate::models::ReportAbout, Error = Error>>;
    fn get_settings_mapping(
        &self,
        settings_mapping_id: &str,
    ) -> Box<dyn Future<Item = crate::models::SettingsMappings, Error = Error>>;
    fn get_settings_notification(
        &self,
        settings_notification_id: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaNotifications, Error = Error>>;
    fn get_settings_reports(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsReports, Error = Error>>;
    fn list_quota_quotas(
        &self,
        enforced: bool,
        include_snapshots: bool,
        zone: &str,
        resume: &str,
        recurse_path_children: bool,
        resolve_names: bool,
        recurse_path_parents: bool,
        persona: &str,
        limit: i32,
        exceeded: bool,
        path: &str,
        _type: &str,
        report_id: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaQuotasExtended, Error = Error>>;
    fn list_quota_reports(
        &self,
        sort: &str,
        resume: &str,
        generated: &str,
        limit: i32,
        _type: &str,
        dir: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaReports, Error = Error>>;
    fn list_settings_mappings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsMappings, Error = Error>>;
    fn list_settings_notifications(
        &self,
    ) -> Box<dyn Future<Item = crate::models::QuotaNotificationsExtended, Error = Error>>;
    fn update_quota_quota(
        &self,
        quota_quota: crate::models::QuotaQuota,
        quota_quota_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_settings_mapping(
        &self,
        settings_mapping: crate::models::SettingsMappingExtended,
        settings_mapping_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_settings_notification(
        &self,
        settings_notification: crate::models::QuotaNotification,
        settings_notification_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_settings_reports(
        &self,
        settings_reports: crate::models::SettingsReportsExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> QuotaApi for QuotaApiClient<C> {
    fn create_quota_quota(
        &self,
        quota_quota: crate::models::QuotaQuotaCreateParams,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("zone", &zone.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/quota/quotas?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &quota_quota,
            hyper::Method::POST,
        )
    }

    fn create_quota_report(
        &self,
        quota_report: crate::models::Empty,
    ) -> Box<dyn Future<Item = crate::models::CreateQuotaReportResponse, Error = Error>> {
        let uri_str = format!("{}/platform/1/quota/reports", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &quota_report,
            hyper::Method::POST,
        )
    }

    fn create_settings_mapping(
        &self,
        settings_mapping: crate::models::SettingsMappingExtendedExtended,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/mappings",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &settings_mapping,
            hyper::Method::POST,
        )
    }

    fn create_settings_notification(
        &self,
        settings_notification: crate::models::QuotaNotificationCreateParams,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/notifications",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &settings_notification,
            hyper::Method::POST,
        )
    }

    fn delete_quota_quota(
        &self,
        quota_quota_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/quotas/{QuotaQuotaId}",
            self.configuration.base_path,
            QuotaQuotaId = quota_quota_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_quota_quotas(
        &self,
        enforced: bool,
        include_snapshots: bool,
        zone: &str,
        recurse_path_children: bool,
        recurse_path_parents: bool,
        persona: &str,
        path: &str,
        _type: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("enforced", &enforced.to_string())
            .append_pair("include_snapshots", &include_snapshots.to_string())
            .append_pair("zone", &zone.to_string())
            .append_pair("recurse_path_children", &recurse_path_children.to_string())
            .append_pair("recurse_path_parents", &recurse_path_parents.to_string())
            .append_pair("persona", &persona.to_string())
            .append_pair("path", &path.to_string())
            .append_pair("type", &_type.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/quota/quotas?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_quota_report(
        &self,
        quota_report_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/reports/{QuotaReportId}",
            self.configuration.base_path,
            QuotaReportId = quota_report_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_settings_mapping(
        &self,
        settings_mapping_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/mappings/{SettingsMappingId}",
            self.configuration.base_path,
            SettingsMappingId = settings_mapping_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_settings_mappings(&self) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/mappings",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_settings_notification(
        &self,
        settings_notification_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/notifications/{SettingsNotificationId}",
            self.configuration.base_path,
            SettingsNotificationId = settings_notification_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_settings_notifications(&self) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/notifications",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn get_quota_license(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LicenseLicense, Error = Error>> {
        let uri_str = format!("{}/platform/5/quota/license", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_quota_quota(
        &self,
        quota_quota_id: &str,
        resolve_names: bool,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaQuotas, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("resolve_names", &resolve_names.to_string())
            .append_pair("zone", &zone.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/quota/quotas/{QuotaQuotaId}?{}",
            self.configuration.base_path,
            q,
            QuotaQuotaId = quota_quota_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_quota_quotas_summary(
        &self,
    ) -> Box<dyn Future<Item = crate::models::QuotaQuotasSummary, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/quotas-summary",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_quota_report(
        &self,
        quota_report_id: &str,
        contents: bool,
    ) -> Box<dyn Future<Item = crate::models::ReportAbout, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("contents", &contents.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/quota/reports/{QuotaReportId}?{}",
            self.configuration.base_path,
            q,
            QuotaReportId = quota_report_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_settings_mapping(
        &self,
        settings_mapping_id: &str,
    ) -> Box<dyn Future<Item = crate::models::SettingsMappings, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/mappings/{SettingsMappingId}",
            self.configuration.base_path,
            SettingsMappingId = settings_mapping_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_settings_notification(
        &self,
        settings_notification_id: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaNotifications, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/notifications/{SettingsNotificationId}",
            self.configuration.base_path,
            SettingsNotificationId = settings_notification_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_settings_reports(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsReports, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/reports",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_quota_quotas(
        &self,
        enforced: bool,
        include_snapshots: bool,
        zone: &str,
        resume: &str,
        recurse_path_children: bool,
        resolve_names: bool,
        recurse_path_parents: bool,
        persona: &str,
        limit: i32,
        exceeded: bool,
        path: &str,
        _type: &str,
        report_id: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaQuotasExtended, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("enforced", &enforced.to_string())
            .append_pair("include_snapshots", &include_snapshots.to_string())
            .append_pair("zone", &zone.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("recurse_path_children", &recurse_path_children.to_string())
            .append_pair("resolve_names", &resolve_names.to_string())
            .append_pair("recurse_path_parents", &recurse_path_parents.to_string())
            .append_pair("persona", &persona.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("exceeded", &exceeded.to_string())
            .append_pair("path", &path.to_string())
            .append_pair("type", &_type.to_string())
            .append_pair("report_id", &report_id.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/quota/quotas?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_quota_reports(
        &self,
        sort: &str,
        resume: &str,
        generated: &str,
        limit: i32,
        _type: &str,
        dir: &str,
    ) -> Box<dyn Future<Item = crate::models::QuotaReports, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("generated", &generated.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("type", &_type.to_string())
            .append_pair("dir", &dir.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/quota/reports?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_settings_mappings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsMappings, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/mappings",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_settings_notifications(
        &self,
    ) -> Box<dyn Future<Item = crate::models::QuotaNotificationsExtended, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/notifications",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_quota_quota(
        &self,
        quota_quota: crate::models::QuotaQuota,
        quota_quota_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/quotas/{QuotaQuotaId}",
            self.configuration.base_path,
            QuotaQuotaId = quota_quota_id
        );
        put(self.configuration.borrow(), &uri_str, &quota_quota)
    }

    fn update_settings_mapping(
        &self,
        settings_mapping: crate::models::SettingsMappingExtended,
        settings_mapping_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/mappings/{SettingsMappingId}",
            self.configuration.base_path,
            SettingsMappingId = settings_mapping_id
        );
        put(self.configuration.borrow(), &uri_str, &settings_mapping)
    }

    fn update_settings_notification(
        &self,
        settings_notification: crate::models::QuotaNotification,
        settings_notification_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/notifications/{SettingsNotificationId}",
            self.configuration.base_path,
            SettingsNotificationId = settings_notification_id
        );
        put(
            self.configuration.borrow(),
            &uri_str,
            &settings_notification,
        )
    }

    fn update_settings_reports(
        &self,
        settings_reports: crate::models::SettingsReportsExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/settings/reports",
            self.configuration.base_path
        );
        put(self.configuration.borrow(), &uri_str, &settings_reports)
    }
}
