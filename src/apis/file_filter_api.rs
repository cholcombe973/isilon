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

pub struct FileFilterApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> FileFilterApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FileFilterApiClient<C> {
        FileFilterApiClient {
            configuration: configuration,
        }
    }
}

pub trait FileFilterApi {
    fn get_file_filter_settings(
        &self,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::FileFilterSettings, Error = Error>>;
    fn update_file_filter_settings(
        &self,
        file_filter_settings: crate::models::FileFilterSettingsExtended,
        zone: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> FileFilterApi for FileFilterApiClient<C> {
    fn get_file_filter_settings(
        &self,
        zone: &str,
    ) -> Box<dyn Future<Item = crate::models::FileFilterSettings, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("zone", &zone.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/file-filter/settings?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_file_filter_settings(
        &self,
        file_filter_settings: crate::models::FileFilterSettingsExtended,
        zone: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("zone", &zone.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/file-filter/settings?{}",
            self.configuration.base_path, q
        );
        put(self.configuration.borrow(), &uri_str, &file_filter_settings)
    }
}
