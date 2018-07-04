/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct ZonesSummaryApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ZonesSummaryApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ZonesSummaryApiClient<C> {
        ZonesSummaryApiClient {
            configuration: configuration,
        }
    }
}

pub trait ZonesSummaryApi {
    fn get_zones_summary(&self, groupnet: &str) -> Box<Future<Item = ::models::ZonesSummaryExtended, Error = Error>>;
    fn get_zones_summary_zone(&self, zones_summary_zone: i32) -> Box<Future<Item = ::models::ZonesSummary, Error = Error>>;
}


impl<C: hyper::client::Connect>ZonesSummaryApi for ZonesSummaryApiClient<C> {
    fn get_zones_summary(&self, groupnet: &str) -> Box<Future<Item = ::models::ZonesSummaryExtended, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("groupnet", &groupnet.to_string())
            .finish();
        let uri_str = format!("{}/platform/1/zones-summary{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ZonesSummaryExtended, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_zones_summary_zone(&self, zones_summary_zone: i32) -> Box<Future<Item = ::models::ZonesSummary, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/platform/1/zones-summary/{ZonesSummaryZone}", configuration.base_path, ZonesSummaryZone=zones_summary_zone);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ZonesSummary, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}