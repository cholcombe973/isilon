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

pub struct RemotesupportApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> RemotesupportApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> RemotesupportApiClient<C> {
        RemotesupportApiClient {
            configuration: configuration,
        }
    }
}

pub trait RemotesupportApi {
    fn get_remotesupport_connectemc(
        &self,
    ) -> Box<dyn Future<Item = crate::models::RemotesupportConnectemc, Error = Error>>;
    fn update_remotesupport_connectemc(
        &self,
        remotesupport_connectemc: crate::models::RemotesupportConnectemcConnectemc,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> RemotesupportApi for RemotesupportApiClient<C> {
    fn get_remotesupport_connectemc(
        &self,
    ) -> Box<dyn Future<Item = crate::models::RemotesupportConnectemc, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/remotesupport/connectemc",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_remotesupport_connectemc(
        &self,
        remotesupport_connectemc: crate::models::RemotesupportConnectemcConnectemc,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/remotesupport/connectemc",
            self.configuration.base_path
        );
        put(
            self.configuration.borrow(),
            &uri_str,
            &remotesupport_connectemc,
        )
    }
}
