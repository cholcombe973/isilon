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

pub struct QuotaReportsApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> QuotaReportsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> QuotaReportsApiClient<C> {
        QuotaReportsApiClient {
            configuration: configuration,
        }
    }
}

pub trait QuotaReportsApi {
    fn get_report_about(
        &self,
        rid: &str,
    ) -> Box<dyn Future<Item = crate::models::ReportAbout, Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> QuotaReportsApi for QuotaReportsApiClient<C> {
    fn get_report_about(
        &self,
        rid: &str,
    ) -> Box<dyn Future<Item = crate::models::ReportAbout, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/quota/reports/{Rid}/about",
            self.configuration.base_path,
            Rid = rid
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }
}
