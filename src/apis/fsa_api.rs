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

pub struct FsaApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> FsaApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FsaApiClient<C> {
        FsaApiClient {
            configuration: configuration,
        }
    }
}

pub trait FsaApi {
    fn delete_fsa_result(&self, fsa_result_id: &str) -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_fsa_settings(&self) -> Box<dyn Future<Item = (), Error = Error>>;
    fn get_fsa_result(
        &self,
        fsa_result_id: &str,
    ) -> Box<dyn Future<Item = crate::models::FsaResults, Error = Error>>;
    fn get_fsa_results(
        &self,
    ) -> Box<dyn Future<Item = crate::models::FsaResultsExtended, Error = Error>>;
    fn get_fsa_settings(
        &self,
        scope: &str,
    ) -> Box<dyn Future<Item = crate::models::FsaSettings, Error = Error>>;
    fn update_fsa_result(
        &self,
        fsa_result: crate::models::FsaResult,
        fsa_result_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_fsa_settings(
        &self,
        fsa_settings: crate::models::FsaSettingsSettings,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> FsaApi for FsaApiClient<C> {
    fn delete_fsa_result(&self, fsa_result_id: &str) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/fsa/results/{FsaResultId}",
            self.configuration.base_path,
            FsaResultId = fsa_result_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_fsa_settings(&self) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!("{}/platform/1/fsa/settings", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn get_fsa_result(
        &self,
        fsa_result_id: &str,
    ) -> Box<dyn Future<Item = crate::models::FsaResults, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/fsa/results/{FsaResultId}",
            self.configuration.base_path,
            FsaResultId = fsa_result_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_fsa_results(
        &self,
    ) -> Box<dyn Future<Item = crate::models::FsaResultsExtended, Error = Error>> {
        let uri_str = format!("{}/platform/3/fsa/results", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_fsa_settings(
        &self,
        scope: &str,
    ) -> Box<dyn Future<Item = crate::models::FsaSettings, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("scope", &scope.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/fsa/settings?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_fsa_result(
        &self,
        fsa_result: crate::models::FsaResult,
        fsa_result_id: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/fsa/results/{FsaResultId}",
            self.configuration.base_path,
            FsaResultId = fsa_result_id
        );
        put(self.configuration.borrow(), &uri_str, &fsa_result)
    }

    fn update_fsa_settings(
        &self,
        fsa_settings: crate::models::FsaSettingsSettings,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!("{}/platform/1/fsa/settings", self.configuration.base_path);

        put(self.configuration.borrow(), &uri_str, &fsa_settings)
    }
}
