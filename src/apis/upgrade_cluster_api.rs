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

pub struct UpgradeClusterApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> UpgradeClusterApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UpgradeClusterApiClient<C> {
        UpgradeClusterApiClient {
            configuration: configuration,
        }
    }
}

pub trait UpgradeClusterApi {
    fn create_nodes_node_patch_sync_item(
        &self,
        nodes_node_patch_sync_item: crate::models::Empty,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn get_nodes_node_firmware_status(
        &self,
        lnn: i32,
        devices: bool,
        package: bool,
    ) -> Box<dyn Future<Item = crate::models::NodesNodeFirmwareStatus, Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> UpgradeClusterApi
    for UpgradeClusterApiClient<C>
{
    fn create_nodes_node_patch_sync_item(
        &self,
        nodes_node_patch_sync_item: crate::models::Empty,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/4/upgrade/cluster/nodes/{Lnn}/patch/sync",
            self.configuration.base_path,
            Lnn = lnn
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &nodes_node_patch_sync_item,
            hyper::Method::POST,
        )
    }

    fn get_nodes_node_firmware_status(
        &self,
        lnn: i32,
        devices: bool,
        package: bool,
    ) -> Box<dyn Future<Item = crate::models::NodesNodeFirmwareStatus, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("devices", &devices.to_string())
            .append_pair("package", &package.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/upgrade/cluster/nodes/{Lnn}/firmware/status?{}",
            self.configuration.base_path,
            q,
            Lnn = lnn
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }
}
