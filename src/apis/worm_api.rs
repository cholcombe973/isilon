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

pub struct WormApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> WormApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> WormApiClient<C> {
        WormApiClient {
            configuration: configuration,
        }
    }
}

pub trait WormApi {
    fn create_worm_domain(
        &self,
        worm_domain: crate::models::WormDomainCreateParams,
    ) -> Result<crate::models::WormDomainExtended, Error>;
    fn get_worm_domain(
        &self,
        worm_domain_id: &str,
    ) -> Result<crate::models::WormDomains, Error>;
    fn get_worm_settings(
        &self,
    ) -> Result<crate::models::WormSettings, Error>;
    fn list_worm_domains(
        &self,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Result<crate::models::WormDomainsExtended, Error>;
    fn update_worm_domain(
        &self,
        worm_domain: crate::models::WormDomain,
        worm_domain_id: &str,
    ) -> Result<(), Error>;
    fn update_worm_settings(
        &self,
        worm_settings: crate::models::WormSettingsExtended,
    ) -> Result<(), Error>;
}

impl<C: hyper::client::connect::Connect + 'static + std::marker::Sync + std::marker::Send + Clone> WormApi for WormApiClient<C> {
    fn create_worm_domain(
        &self,
        worm_domain: crate::models::WormDomainCreateParams,
    ) -> Result<crate::models::WormDomainExtended, Error> {
        let uri_str = format!("{}/platform/1/worm/domains", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &worm_domain,
            hyper::Method::POST,
        )
    }

    fn get_worm_domain(
        &self,
        worm_domain_id: &str,
    ) -> Result<crate::models::WormDomains, Error> {
        let uri_str = format!(
            "{}/platform/1/worm/domains/{WormDomainId}",
            self.configuration.base_path,
            WormDomainId = worm_domain_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_worm_settings(
        &self,
    ) -> Result<crate::models::WormSettings, Error> {
        let uri_str = format!("{}/platform/1/worm/settings", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_worm_domains(
        &self,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Result<crate::models::WormDomainsExtended, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .append_pair("resume", &resume.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/worm/domains?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_worm_domain(
        &self,
        worm_domain: crate::models::WormDomain,
        worm_domain_id: &str,
    ) -> Result<(), Error> {
        let uri_str = format!(
            "{}/platform/1/worm/domains/{WormDomainId}",
            self.configuration.base_path,
            WormDomainId = worm_domain_id
        );
        put(self.configuration.borrow(), &uri_str, &worm_domain)
    }

    fn update_worm_settings(
        &self,
        worm_settings: crate::models::WormSettingsExtended,
    ) -> Result<(), Error> {
        let uri_str = format!("{}/platform/1/worm/settings", self.configuration.base_path);
        put(self.configuration.borrow(), &uri_str, &worm_settings)
    }
}
