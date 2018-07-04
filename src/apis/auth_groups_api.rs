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

pub struct AuthGroupsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AuthGroupsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthGroupsApiClient<C> {
        AuthGroupsApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthGroupsApi {
    fn create_group_member(&self, group_member: ::models::AuthAccessAccessItemFileGroup, group: &str, zone: &str, provider: &str) -> Box<Future<Item = ::models::CreateResponse, Error = Error>>;
    fn delete_group_member(&self, group_member_id: &str, group: &str, zone: &str, provider: &str) -> Box<Future<Item = (), Error = Error>>;
    fn list_group_members(&self, group: &str, resolve_names: bool, resume: &str, limit: i32, zone: &str, provider: &str) -> Box<Future<Item = ::models::GroupMembers, Error = Error>>;
}


impl<C: hyper::client::Connect>AuthGroupsApi for AuthGroupsApiClient<C> {
    fn create_group_member(&self, group_member: ::models::AuthAccessAccessItemFileGroup, group: &str, zone: &str, provider: &str) -> Box<Future<Item = ::models::CreateResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("zone", &zone.to_string())
            .append_pair("provider", &provider.to_string())
            .finish();
        let uri_str = format!("{}/platform/1/auth/groups/{Group}/members{}", configuration.base_path, query, Group=group);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&group_member).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::CreateResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_group_member(&self, group_member_id: &str, group: &str, zone: &str, provider: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("zone", &zone.to_string())
            .append_pair("provider", &provider.to_string())
            .finish();
        let uri_str = format!("{}/platform/1/auth/groups/{Group}/members/{GroupMemberId}{}", configuration.base_path, query, GroupMemberId=group_member_id, Group=group);

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
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn list_group_members(&self, group: &str, resolve_names: bool, resume: &str, limit: i32, zone: &str, provider: &str) -> Box<Future<Item = ::models::GroupMembers, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("resolve_names", &resolve_names.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("zone", &zone.to_string())
            .append_pair("provider", &provider.to_string())
            .finish();
        let uri_str = format!("{}/platform/1/auth/groups/{Group}/members{}", configuration.base_path, query, Group=group);

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
                let parsed: Result<::models::GroupMembers, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}