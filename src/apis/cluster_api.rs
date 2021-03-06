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

pub struct ClusterApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + 'static> ClusterApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ClusterApiClient<C> {
        ClusterApiClient {
            configuration: configuration,
        }
    }
}

pub trait ClusterApi {
    fn create_cluster_add_node_item(
        &self,
        cluster_add_node_item: crate::models::ClusterAddNodeItem,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_diagnostics_gather_start_item(
        &self,
        diagnostics_gather_start_item: crate::models::DiagnosticsGatherSettingsExtended,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_diagnostics_gather_stop_item(
        &self,
        diagnostics_gather_stop_item: crate::models::Empty,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_diagnostics_netlogger_start_item(
        &self,
        diagnostics_netlogger_start_item: crate::models::DiagnosticsNetloggerSettings,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_diagnostics_netlogger_stop_item(
        &self,
        diagnostics_netlogger_stop_item: crate::models::Empty,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn get_cluster_config(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterConfig, Error = Error>>;
    fn get_cluster_email(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterEmail, Error = Error>>;
    fn get_cluster_external_ips(&self) -> Box<dyn Future<Item = Vec<String>, Error = Error>>;
    fn get_cluster_identity(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterIdentity, Error = Error>>;
    fn get_cluster_node(
        &self,
        cluster_node_id: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::ClusterNodesExtendedExtended, Error = Error>>;
    fn get_cluster_nodes(
        &self,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::ClusterNodesExtendedExtended, Error = Error>>;
    fn get_cluster_nodes_available(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterNodesAvailable, Error = Error>>;
    fn get_cluster_owner(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterOwner, Error = Error>>;
    fn get_cluster_statfs(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterStatfs, Error = Error>>;
    fn get_cluster_time(&self)
        -> Box<dyn Future<Item = crate::models::ClusterTime, Error = Error>>;
    fn get_cluster_timezone(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterTimezone, Error = Error>>;
    fn get_cluster_version(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterVersion, Error = Error>>;
    fn get_diagnostics_gather(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsGatherStatus, Error = Error>>;
    fn get_diagnostics_gather_settings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsGatherSettings, Error = Error>>;
    fn get_diagnostics_gather_status(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsGatherStatus, Error = Error>>;
    fn get_diagnostics_netlogger(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsNetloggerStatus, Error = Error>>;
    fn get_diagnostics_netlogger_settings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsNetloggerSettings, Error = Error>>;
    fn get_diagnostics_netlogger_status(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsNetloggerStatus, Error = Error>>;
    fn get_timezone_region(
        &self,
        timezone_region_id: &str,
        sort: &str,
        resume: &str,
        show_all: bool,
        dst_reset: bool,
        limit: i32,
        dir: &str,
    ) -> Box<dyn Future<Item = crate::models::TimezoneRegions, Error = Error>>;
    fn get_timezone_settings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::TimezoneSettings, Error = Error>>;
    fn update_cluster_email(
        &self,
        cluster_email: crate::models::ClusterEmailExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_cluster_node(
        &self,
        cluster_node: crate::models::ClusterNode,
        cluster_node_id: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_cluster_owner(
        &self,
        cluster_owner: crate::models::ClusterOwner,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_cluster_time(
        &self,
        cluster_time: crate::models::ClusterTimeExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_cluster_timezone(
        &self,
        cluster_timezone: crate::models::ClusterTimezoneExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_diagnostics_gather_settings(
        &self,
        diagnostics_gather_settings: crate::models::DiagnosticsGatherSettingsExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_diagnostics_netlogger_settings(
        &self,
        diagnostics_netlogger_settings: crate::models::DiagnosticsNetloggerSettings,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_timezone_settings(
        &self,
        timezone_settings: crate::models::TimezoneRegionTimezone,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> ClusterApi for ClusterApiClient<C> {
    fn create_cluster_add_node_item(
        &self,
        cluster_add_node_item: crate::models::ClusterAddNodeItem,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/add-node",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &cluster_add_node_item,
            hyper::Method::POST,
        )
    }

    fn create_diagnostics_gather_start_item(
        &self,
        diagnostics_gather_start_item: crate::models::DiagnosticsGatherSettingsExtended,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/gather/start",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &diagnostics_gather_start_item,
            hyper::Method::POST,
        )
    }

    fn create_diagnostics_gather_stop_item(
        &self,
        diagnostics_gather_stop_item: crate::models::Empty,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/gather/stop",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &diagnostics_gather_stop_item,
            hyper::Method::POST,
        )
    }

    fn create_diagnostics_netlogger_start_item(
        &self,
        diagnostics_netlogger_start_item: crate::models::DiagnosticsNetloggerSettings,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/netlogger/start",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &diagnostics_netlogger_start_item,
            hyper::Method::POST,
        )
    }

    fn create_diagnostics_netlogger_stop_item(
        &self,
        diagnostics_netlogger_stop_item: crate::models::Empty,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/netlogger/stop",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &diagnostics_netlogger_stop_item,
            hyper::Method::POST,
        )
    }

    fn get_cluster_config(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterConfig, Error = Error>> {
        let uri_str = format!("{}/platform/3/cluster/config", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_email(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterEmail, Error = Error>> {
        let uri_str = format!("{}/platform/1/cluster/email", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_external_ips(&self) -> Box<dyn Future<Item = Vec<String>, Error = Error>> {
        let uri_str = format!(
            "{}/platform/2/cluster/external-ips",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_identity(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterIdentity, Error = Error>> {
        let uri_str = format!(
            "{}/platform/5/cluster/identity",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_node(
        &self,
        cluster_node_id: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::ClusterNodesExtendedExtended, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{ClusterNodeId}?{}",
            self.configuration.base_path,
            q,
            ClusterNodeId = cluster_node_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_nodes(
        &self,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::ClusterNodesExtendedExtended, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_nodes_available(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterNodesAvailable, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes-available",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_owner(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterOwner, Error = Error>> {
        let uri_str = format!("{}/platform/1/cluster/owner", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_statfs(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterStatfs, Error = Error>> {
        let uri_str = format!("{}/platform/1/cluster/statfs", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_time(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterTime, Error = Error>> {
        let uri_str = format!("{}/platform/3/cluster/time", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_timezone(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterTimezone, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/timezone",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_cluster_version(
        &self,
    ) -> Box<dyn Future<Item = crate::models::ClusterVersion, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/version",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_diagnostics_gather(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsGatherStatus, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/gather",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_diagnostics_gather_settings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsGatherSettings, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/gather/settings",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_diagnostics_gather_status(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsGatherStatus, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/gather/status",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_diagnostics_netlogger(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsNetloggerStatus, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/netlogger",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_diagnostics_netlogger_settings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsNetloggerSettings, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/netlogger/settings",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_diagnostics_netlogger_status(
        &self,
    ) -> Box<dyn Future<Item = crate::models::DiagnosticsNetloggerStatus, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/diagnostics/netlogger/status",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_timezone_region(
        &self,
        timezone_region_id: &str,
        sort: &str,
        resume: &str,
        show_all: bool,
        dst_reset: bool,
        limit: i32,
        dir: &str,
    ) -> Box<dyn Future<Item = crate::models::TimezoneRegions, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("show_all", &show_all.to_string())
            .append_pair("dst_reset", &dst_reset.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/cluster/timezone/regions/{TimezoneRegionId}?{}",
            self.configuration.base_path,
            q,
            TimezoneRegionId = timezone_region_id
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_timezone_settings(
        &self,
    ) -> Box<dyn Future<Item = crate::models::TimezoneSettings, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/timezone/settings",
            self.configuration.base_path
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_cluster_email(
        &self,
        cluster_email: crate::models::ClusterEmailExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!("{}/platform/1/cluster/email", self.configuration.base_path);
        put(self.configuration.borrow(), &uri, &cluster_email)
    }

    fn update_cluster_node(
        &self,
        cluster_node: crate::models::ClusterNode,
        cluster_node_id: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!(
            "{}/platform/5/cluster/nodes/{ClusterNodeId}",
            self.configuration.base_path,
            ClusterNodeId = cluster_node_id
        );
        put(self.configuration.borrow(), &uri, &cluster_node)
    }

    fn update_cluster_owner(
        &self,
        cluster_owner: crate::models::ClusterOwner,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!("{}/platform/1/cluster/owner", self.configuration.base_path);
        put(self.configuration.borrow(), &uri, &cluster_owner)
    }

    fn update_cluster_time(
        &self,
        cluster_time: crate::models::ClusterTimeExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!("{}/platform/3/cluster/time", self.configuration.base_path);
        put(self.configuration.borrow(), &uri, &cluster_time)
    }

    fn update_cluster_timezone(
        &self,
        cluster_timezone: crate::models::ClusterTimezoneExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!(
            "{}/platform/3/cluster/timezone",
            self.configuration.base_path
        );
        put(self.configuration.borrow(), &uri, &cluster_timezone)
    }

    fn update_diagnostics_gather_settings(
        &self,
        diagnostics_gather_settings: crate::models::DiagnosticsGatherSettingsExtended,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!(
            "{}/platform/3/cluster/diagnostics/gather/settings",
            self.configuration.base_path
        );
        put(
            self.configuration.borrow(),
            &uri,
            &diagnostics_gather_settings,
        )
    }

    fn update_diagnostics_netlogger_settings(
        &self,
        diagnostics_netlogger_settings: crate::models::DiagnosticsNetloggerSettings,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!(
            "{}/platform/3/cluster/diagnostics/netlogger/settings",
            self.configuration.base_path
        );
        put(
            self.configuration.borrow(),
            &uri,
            &diagnostics_netlogger_settings,
        )
    }

    fn update_timezone_settings(
        &self,
        timezone_settings: crate::models::TimezoneRegionTimezone,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri = format!(
            "{}/platform/3/cluster/timezone/settings",
            self.configuration.base_path
        );
        put(self.configuration.borrow(), &uri, &timezone_settings)
    }
}
