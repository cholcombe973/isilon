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

pub struct AuthRolesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AuthRolesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthRolesApiClient<C> {
        AuthRolesApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthRolesApi {
    fn create_role_member(&self, role_member: ::models::AuthAccessAccessItemFileGroup, role: &str) -> Box<Future<Item = ::models::CreateResponse, Error = Error>>;
    fn create_role_privilege(&self, role_privilege: ::models::AuthIdNtokenPrivilegeItem, role: &str) -> Box<Future<Item = ::models::CreateResponse, Error = Error>>;
    fn delete_role_member(&self, role_member_id: &str, role: &str) -> Box<Future<Item = (), Error = Error>>;
    fn delete_role_privilege(&self, role_privilege_id: &str, role: &str) -> Box<Future<Item = (), Error = Error>>;
    fn list_role_members(&self, role: &str, resolve_names: bool) -> Box<Future<Item = ::models::GroupMembers, Error = Error>>;
    fn list_role_privileges(&self, role: &str) -> Box<Future<Item = ::models::RolePrivileges, Error = Error>>;
}


impl<C: hyper::client::Connect>AuthRolesApi for AuthRolesApiClient<C> {
    fn create_role_member(&self, role_member: ::models::AuthAccessAccessItemFileGroup, role: &str) -> Box<Future<Item = ::models::CreateResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/platform/1/auth/roles/{Role}/members", configuration.base_path, Role=role);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&role_member).unwrap();
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

    fn create_role_privilege(&self, role_privilege: ::models::AuthIdNtokenPrivilegeItem, role: &str) -> Box<Future<Item = ::models::CreateResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/platform/1/auth/roles/{Role}/privileges", configuration.base_path, Role=role);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&role_privilege).unwrap();
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

    fn delete_role_member(&self, role_member_id: &str, role: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let uri_str = format!("{}/platform/1/auth/roles/{Role}/members/{RoleMemberId}", configuration.base_path, RoleMemberId=role_member_id, Role=role);

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

    fn delete_role_privilege(&self, role_privilege_id: &str, role: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let uri_str = format!("{}/platform/1/auth/roles/{Role}/privileges/{RolePrivilegeId}", configuration.base_path, RolePrivilegeId=role_privilege_id, Role=role);

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

    fn list_role_members(&self, role: &str, resolve_names: bool) -> Box<Future<Item = ::models::GroupMembers, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("resolve_names", &resolve_names.to_string())
            .finish();
        let uri_str = format!("{}/platform/1/auth/roles/{Role}/members{}", configuration.base_path, query, Role=role);

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

    fn list_role_privileges(&self, role: &str) -> Box<Future<Item = ::models::RolePrivileges, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/platform/1/auth/roles/{Role}/privileges", configuration.base_path, Role=role);

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
                let parsed: Result<::models::RolePrivileges, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}