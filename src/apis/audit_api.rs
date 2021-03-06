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

pub struct AuditApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> AuditApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuditApiClient<C> {
        AuditApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuditApi {
    fn create_audit_topic(
        &self,
        audit_topic: crate::models::AuditTopicCreateParams,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>>;
    fn delete_audit_topic(&self, audit_topic_id: &str)
        -> Box<dyn Future<Item = (), Error = Error>>;
    fn get_audit_progress(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::AuditProgress, Error = Error>>;
    fn get_audit_settings(
        &self,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::AuditSettings, Error = Error>>;
    fn get_audit_topic(
        &self,
        audit_topic_id: &str,
    ) -> Box<dyn Future<Item = crate::models::AuditTopics, Error = Error>>;
    fn get_progress_global(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ProgressGlobal, Error = Error>>;
    fn get_settings_global(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsGlobalExtended, Error = Error>>;
    fn list_audit_topics(
        &self,
    ) -> Box<dyn Future<Item = crate::models::AuditTopicsExtended, Error = Error>>;
    fn update_audit_settings(
        &self,
        audit_settings: crate::models::AuditSettingsSettings,
        zone: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_audit_topic(
        &self,
        audit_topic: crate::models::AuditTopic,
        audit_topic_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_settings_global(
        &self,
        settings_global: crate::models::SettingsGlobalSettings,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> AuditApi for AuditApiClient<C> {
    fn create_audit_topic(
        &self,
        audit_topic: crate::models::AuditTopicCreateParams,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>> {
        let uri_str = format!("{}/platform/1/audit/topics", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &audit_topic,
            hyper::Method::POST,
        )
    }

    fn delete_audit_topic(
        &self,
        audit_topic_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/audit/topics/{AuditTopicId}",
            self.configuration.base_path,
            AuditTopicId = audit_topic_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn get_audit_progress(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::AuditProgress, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("lnn", &lnn.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/4/audit/progress?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_audit_settings(
        &self,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::AuditSettings, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("zone", &zone.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/audit/settings?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_audit_topic(
        &self,
        audit_topic_id: &str,
    ) -> Box<dyn Future<Item = crate::models::AuditTopics, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/audit/topics/{AuditTopicId}",
            self.configuration.base_path,
            AuditTopicId = audit_topic_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_progress_global(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ProgressGlobal, Error = Error>> {
        let uri_str = format!(
            "{}/platform/4/audit/progress/global",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_settings_global(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsGlobalExtended, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/audit/settings/global",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_audit_topics(
        &self,
    ) -> Box<dyn Future<Item = crate::models::AuditTopicsExtended, Error = Error>> {
        let uri_str = format!("{}/platform/1/audit/topics", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_audit_settings(
        &self,
        audit_settings: crate::models::AuditSettingsSettings,
        zone: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("zone", &zone.to_string())
            .finish();
        let uri = format!(
            "{}/platform/3/audit/settings?{}",
            self.configuration.base_path, q
        );
        put(self.configuration.borrow(), &uri, &audit_settings)
    }

    fn update_audit_topic(
        &self,
        audit_topic: crate::models::AuditTopic,
        audit_topic_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!(
            "{}/platform/1/audit/topics/{AuditTopicId}",
            self.configuration.base_path,
            AuditTopicId = audit_topic_id
        );
        put(self.configuration.borrow(), &uri, &audit_topic)
    }

    fn update_settings_global(
        &self,
        settings_global: crate::models::SettingsGlobalSettings,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!(
            "{}/platform/3/audit/settings/global",
            self.configuration.base_path
        );
        put(self.configuration.borrow(), &uri, &settings_global)
    }
}
