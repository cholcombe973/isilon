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

use super::{configuration, query, Error};

pub struct LocalApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> LocalApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> LocalApiClient<C> {
        LocalApiClient {
            configuration: configuration,
        }
    }
}

pub trait LocalApi {
    fn get_cluster_time(
        &self,
    ) -> Result<crate::models::ClusterTimeExtendedExtended, Error>;
}

impl<C: hyper::client::connect::Connect + 'static + std::marker::Sync + std::marker::Send + Clone> LocalApi for LocalApiClient<C> {
    fn get_cluster_time(
        &self,
    ) -> Result<crate::models::ClusterTimeExtendedExtended, Error> {
        let uri_str = format!(
            "{}/platform/3/local/cluster/time",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }
}
