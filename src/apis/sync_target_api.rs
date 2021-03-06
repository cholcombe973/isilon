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

pub struct SyncTargetApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> SyncTargetApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SyncTargetApiClient<C> {
        SyncTargetApiClient {
            configuration: configuration,
        }
    }
}

pub trait SyncTargetApi {
    fn create_policies_policy_cancel_item(
        &self,
        policies_policy_cancel_item: crate::models::Empty,
        policy: &str,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>>;
    fn get_reports_report_subreport(
        &self,
        reports_report_subreport_id: &str,
        rid: &str,
    ) -> Box<dyn Future<Item = crate::models::ReportsReportSubreports, Error = Error>>;
    fn get_reports_report_subreports(
        &self,
        rid: &str,
        sort: &str,
        resume: &str,
        newer_than: i32,
        state: &str,
        limit: i32,
        dir: &str,
    ) -> Box<dyn Future<Item = crate::models::ReportsReportSubreportsExtended, Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> SyncTargetApi for SyncTargetApiClient<C> {
    fn create_policies_policy_cancel_item(
        &self,
        policies_policy_cancel_item: crate::models::Empty,
        policy: &str,
    ) -> Box<dyn Future<Item = crate::models::CreateResponse, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/sync/target/policies/{Policy}/cancel",
            self.configuration.base_path,
            Policy = policy
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &policies_policy_cancel_item,
            hyper::Method::POST,
        )
    }

    fn get_reports_report_subreport(
        &self,
        reports_report_subreport_id: &str,
        rid: &str,
    ) -> Box<dyn Future<Item = crate::models::ReportsReportSubreports, Error = Error>> {
        let uri_str = format!(
            "{}/platform/4/sync/target/reports/{Rid}/subreports/{ReportsReportSubreportId}",
            self.configuration.base_path,
            ReportsReportSubreportId = reports_report_subreport_id,
            Rid = rid
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_reports_report_subreports(
        &self,
        rid: &str,
        sort: &str,
        resume: &str,
        newer_than: i32,
        state: &str,
        limit: i32,
        dir: &str,
    ) -> Box<dyn Future<Item = crate::models::ReportsReportSubreportsExtended, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("newer_than", &newer_than.to_string())
            .append_pair("state", &state.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/4/sync/target/reports/{Rid}/subreports?{}",
            self.configuration.base_path,
            q,
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
