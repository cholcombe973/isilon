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

pub struct FilesystemApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> FilesystemApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FilesystemApiClient<C> {
        FilesystemApiClient {
            configuration: configuration,
        }
    }
}

pub trait FilesystemApi {
    fn get_settings_access_time(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsAccessTime, Error = Error>>;
    fn get_settings_character_encodings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsCharacterEncodings, Error = Error>>;
    fn update_settings_access_time(
        &self,
        settings_access_time: crate::models::SettingsAccessTimeExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_settings_character_encodings(
        &self,
        settings_character_encodings: crate::models::SettingsCharacterEncodingsExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> FilesystemApi for FilesystemApiClient<C> {
    fn get_settings_access_time(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsAccessTime, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/filesystem/settings/access-time",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_settings_character_encodings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::SettingsCharacterEncodings, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/filesystem/settings/character-encodings",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_settings_access_time(
        &self,
        settings_access_time: crate::models::SettingsAccessTimeExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/filesystem/settings/access-time",
            self.configuration.base_path
        );
        put(self.configuration.borrow(), &uri_str, &settings_access_time)
    }

    fn update_settings_character_encodings(
        &self,
        settings_character_encodings: crate::models::SettingsCharacterEncodingsExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/filesystem/settings/character-encodings",
            self.configuration.base_path
        );
        put(
            self.configuration.borrow(),
            &uri_str,
            &settings_character_encodings,
        )
    }
}
