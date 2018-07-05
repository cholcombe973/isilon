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

pub struct NetworkGroupnetsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NetworkGroupnetsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NetworkGroupnetsApiClient<C> {
        NetworkGroupnetsApiClient {
            configuration: configuration,
        }
    }
}

pub trait NetworkGroupnetsApi {
    fn create_groupnet_subnet(
        &self,
        groupnet_subnet: ::models::GroupnetSubnetCreateParams,
        groupnet: &str,
    ) -> Box<Future<Item = ::models::CreateResponse, Error = Error>>;
    fn create_subnets_subnet_pool(
        &self,
        subnets_subnet_pool: ::models::SubnetsSubnetPoolCreateParams,
        groupnet: &str,
        subnet: &str,
        force: bool,
    ) -> Box<Future<Item = ::models::CreateResponse, Error = Error>>;
    fn delete_groupnet_subnet(
        &self,
        groupnet_subnet_id: &str,
        groupnet: &str,
        force: bool,
    ) -> Box<Future<Item = (), Error = Error>>;
    fn delete_subnets_subnet_pool(
        &self,
        subnets_subnet_pool_id: &str,
        groupnet: &str,
        subnet: &str,
    ) -> Box<Future<Item = (), Error = Error>>;
    fn get_groupnet_subnet(
        &self,
        groupnet_subnet_id: &str,
        groupnet: &str,
    ) -> Box<Future<Item = ::models::GroupnetSubnets, Error = Error>>;
    fn get_subnets_subnet_pool(
        &self,
        subnets_subnet_pool_id: &str,
        groupnet: &str,
        subnet: &str,
    ) -> Box<Future<Item = ::models::SubnetsSubnetPools, Error = Error>>;
    fn list_groupnet_subnets(
        &self,
        groupnet: &str,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Box<Future<Item = ::models::GroupnetSubnetsExtended, Error = Error>>;
    fn list_subnets_subnet_pools(
        &self,
        groupnet: &str,
        subnet: &str,
        sort: &str,
        resume: &str,
        access_zone: &str,
        alloc_method: &str,
        limit: i32,
        dir: &str,
    ) -> Box<Future<Item = ::models::SubnetsSubnetPoolsExtended, Error = Error>>;
    fn update_groupnet_subnet(
        &self,
        groupnet_subnet: ::models::GroupnetSubnet,
        groupnet_subnet_id: &str,
        groupnet: &str,
        force: bool,
    ) -> Box<Future<Item = (), Error = Error>>;
    fn update_subnets_subnet_pool(
        &self,
        subnets_subnet_pool: ::models::SubnetsSubnetPool,
        subnets_subnet_pool_id: &str,
        groupnet: &str,
        subnet: &str,
        force: bool,
    ) -> Box<Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::Connect> NetworkGroupnetsApi for NetworkGroupnetsApiClient<C> {
    fn create_groupnet_subnet(
        &self,
        groupnet_subnet: ::models::GroupnetSubnetCreateParams,
        groupnet: &str,
    ) -> Box<Future<Item = ::models::CreateResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!(
            "{}/platform/4/network/groupnets/{Groupnet}/subnets",
            configuration.base_path,
            Groupnet = groupnet
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        let serialized = serde_json::to_string(&groupnet_subnet).unwrap();
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
                    let parsed: Result<::models::CreateResponse, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn create_subnets_subnet_pool(
        &self,
        subnets_subnet_pool: ::models::SubnetsSubnetPoolCreateParams,
        groupnet: &str,
        subnet: &str,
        force: bool,
    ) -> Box<Future<Item = ::models::CreateResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("force", &force.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools{}",
            configuration.base_path,
            query,
            Groupnet = groupnet,
            Subnet = subnet
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        let serialized = serde_json::to_string(&subnets_subnet_pool).unwrap();
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
                    let parsed: Result<::models::CreateResponse, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn delete_groupnet_subnet(
        &self,
        groupnet_subnet_id: &str,
        groupnet: &str,
        force: bool,
    ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("force", &force.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/4/network/groupnets/{Groupnet}/subnets/{GroupnetSubnetId}{}",
            configuration.base_path,
            query,
            GroupnetSubnetId = groupnet_subnet_id,
            Groupnet = groupnet
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

    fn delete_subnets_subnet_pool(
        &self,
        subnets_subnet_pool_id: &str,
        groupnet: &str,
        subnet: &str,
    ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let uri_str = format!("{}/platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{SubnetsSubnetPoolId}", configuration.base_path, SubnetsSubnetPoolId=subnets_subnet_pool_id, Groupnet=groupnet, Subnet=subnet);

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

    fn get_groupnet_subnet(
        &self,
        groupnet_subnet_id: &str,
        groupnet: &str,
    ) -> Box<Future<Item = ::models::GroupnetSubnets, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!(
            "{}/platform/4/network/groupnets/{Groupnet}/subnets/{GroupnetSubnetId}",
            configuration.base_path,
            GroupnetSubnetId = groupnet_subnet_id,
            Groupnet = groupnet
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
                    let parsed: Result<::models::GroupnetSubnets, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn get_subnets_subnet_pool(
        &self,
        subnets_subnet_pool_id: &str,
        groupnet: &str,
        subnet: &str,
    ) -> Box<Future<Item = ::models::SubnetsSubnetPools, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{SubnetsSubnetPoolId}", configuration.base_path, SubnetsSubnetPoolId=subnets_subnet_pool_id, Groupnet=groupnet, Subnet=subnet);

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
                    let parsed: Result<::models::SubnetsSubnetPools, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn list_groupnet_subnets(
        &self,
        groupnet: &str,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Box<Future<Item = ::models::GroupnetSubnetsExtended, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .append_pair("resume", &resume.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/4/network/groupnets/{Groupnet}/subnets{}",
            configuration.base_path,
            query,
            Groupnet = groupnet
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
                    let parsed: Result<::models::GroupnetSubnetsExtended, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn list_subnets_subnet_pools(
        &self,
        groupnet: &str,
        subnet: &str,
        sort: &str,
        resume: &str,
        access_zone: &str,
        alloc_method: &str,
        limit: i32,
        dir: &str,
    ) -> Box<Future<Item = ::models::SubnetsSubnetPoolsExtended, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("access_zone", &access_zone.to_string())
            .append_pair("alloc_method", &alloc_method.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools{}",
            configuration.base_path,
            query,
            Groupnet = groupnet,
            Subnet = subnet
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
                    let parsed: Result<::models::SubnetsSubnetPoolsExtended, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                })
                .map_err(|e| Error::from(e)),
        )
    }

    fn update_groupnet_subnet(
        &self,
        groupnet_subnet: ::models::GroupnetSubnet,
        groupnet_subnet_id: &str,
        groupnet: &str,
        force: bool,
    ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("force", &force.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/4/network/groupnets/{Groupnet}/subnets/{GroupnetSubnetId}{}",
            configuration.base_path,
            query,
            GroupnetSubnetId = groupnet_subnet_id,
            Groupnet = groupnet
        );

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        let serialized = serde_json::to_string(&groupnet_subnet).unwrap();
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

    fn update_subnets_subnet_pool(
        &self,
        subnets_subnet_pool: ::models::SubnetsSubnetPool,
        subnets_subnet_pool_id: &str,
        groupnet: &str,
        subnet: &str,
        force: bool,
    ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("force", &force.to_string())
            .finish();
        let uri_str = format!("{}/platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{SubnetsSubnetPoolId}{}", configuration.base_path, query, SubnetsSubnetPoolId=subnets_subnet_pool_id, Groupnet=groupnet, Subnet=subnet);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());
        configuration.set_session(&mut req).unwrap();

        let serialized = serde_json::to_string(&subnets_subnet_pool).unwrap();
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
