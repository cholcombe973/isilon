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
use futures::{Future, Stream};
use hyper;
use serde_json;

use super::{configuration, Error};

pub struct FilepoolApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> FilepoolApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FilepoolApiClient<C> {
        FilepoolApiClient {
            configuration: configuration,
        }
    }
}

pub trait FilepoolApi {
    fn create_filepool_policy(
        &self,
        filepool_policy: ::models::FilepoolPolicyCreateParams,
    ) -> Box<Future<Item = ::models::CreateFilepoolPolicyResponse, Error = Error>>;
    fn delete_filepool_policy(
        &self,
        filepool_policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error>>;
    fn get_filepool_default_policy(
        &self,
    ) -> Box<Future<Item = ::models::FilepoolDefaultPolicy, Error = Error>>;
    fn get_filepool_policy(
        &self,
        filepool_policy_id: &str,
    ) -> Box<Future<Item = ::models::FilepoolPolicies, Error = Error>>;
    fn get_filepool_template(
        &self,
        filepool_template_id: &str,
    ) -> Box<Future<Item = ::models::FilepoolTemplates, Error = Error>>;
    fn get_filepool_templates(
        &self,
    ) -> Box<Future<Item = ::models::FilepoolTemplates, Error = Error>>;
    fn list_filepool_policies(
        &self,
    ) -> Box<Future<Item = ::models::FilepoolPoliciesExtended, Error = Error>>;
    fn update_filepool_default_policy(
        &self,
        filepool_default_policy: ::models::FilepoolDefaultPolicyExtended,
    ) -> Box<Future<Item = (), Error = Error>>;
    fn update_filepool_policy(
        &self,
        filepool_policy: ::models::FilepoolPolicy,
        filepool_policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::Connect> FilepoolApi for FilepoolApiClient<C> {
    fn create_filepool_policy(
        &self,
        filepool_policy: ::models::FilepoolPolicyCreateParams,
    ) -> Box<Future<Item = ::models::CreateFilepoolPolicyResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/platform/4/filepool/policies", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        let serialized = serde_json::to_string(&filepool_policy).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|body| {
                    let parsed: Result<
                        ::models::CreateFilepoolPolicyResponse,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn delete_filepool_policy(
        &self,
        filepool_policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let uri_str = format!(
            "{}/platform/4/filepool/policies/{FilepoolPolicyId}",
            configuration.base_path,
            FilepoolPolicyId = filepool_policy_id
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|_| futures::future::ok(())),
        )
    }

    fn get_filepool_default_policy(
        &self,
    ) -> Box<Future<Item = ::models::FilepoolDefaultPolicy, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!(
            "{}/platform/4/filepool/default-policy",
            configuration.base_path
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|body| {
                    let parsed: Result<::models::FilepoolDefaultPolicy, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn get_filepool_policy(
        &self,
        filepool_policy_id: &str,
    ) -> Box<Future<Item = ::models::FilepoolPolicies, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!(
            "{}/platform/4/filepool/policies/{FilepoolPolicyId}",
            configuration.base_path,
            FilepoolPolicyId = filepool_policy_id
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|body| {
                    let parsed: Result<::models::FilepoolPolicies, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn get_filepool_template(
        &self,
        filepool_template_id: &str,
    ) -> Box<Future<Item = ::models::FilepoolTemplates, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!(
            "{}/platform/4/filepool/templates/{FilepoolTemplateId}",
            configuration.base_path,
            FilepoolTemplateId = filepool_template_id
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|body| {
                    let parsed: Result<::models::FilepoolTemplates, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn get_filepool_templates(
        &self,
    ) -> Box<Future<Item = ::models::FilepoolTemplates, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/platform/4/filepool/templates", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|body| {
                    let parsed: Result<::models::FilepoolTemplates, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn list_filepool_policies(
        &self,
    ) -> Box<Future<Item = ::models::FilepoolPoliciesExtended, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/platform/4/filepool/policies", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|body| {
                    let parsed: Result<::models::FilepoolPoliciesExtended, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn update_filepool_default_policy(
        &self,
        filepool_default_policy: ::models::FilepoolDefaultPolicyExtended,
    ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let uri_str = format!(
            "{}/platform/4/filepool/default-policy",
            configuration.base_path
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        let serialized = serde_json::to_string(&filepool_default_policy).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|_| futures::future::ok(())),
        )
    }

    fn update_filepool_policy(
        &self,
        filepool_policy: ::models::FilepoolPolicy,
        filepool_policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let uri_str = format!(
            "{}/platform/4/filepool/policies/{FilepoolPolicyId}",
            configuration.base_path,
            FilepoolPolicyId = filepool_policy_id
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        let serialized = serde_json::to_string(&filepool_policy).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .and_then(|res| res.body().concat2())
                .map_err(|e| Error::from(e))
                .and_then(|_| futures::future::ok(())),
        )
    }
}
