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

pub struct ClusterNodesApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> ClusterNodesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ClusterNodesApiClient<C> {
        ClusterNodesApiClient {
            configuration: configuration,
        }
    }
}

pub trait ClusterNodesApi {
    fn create_drives_drive_add_item(
        &self,
        drives_drive_add_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_drives_drive_firmware_update_item(
        &self,
        drives_drive_firmware_update_item: crate::models::DrivesDriveFirmwareUpdateItem,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_drives_drive_format_item(
        &self,
        drives_drive_format_item: crate::models::DrivesDriveFormatItem,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_drives_drive_purpose_item(
        &self,
        drives_drive_purpose_item: crate::models::DrivesDrivePurposeItem,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_drives_drive_smartfail_item(
        &self,
        drives_drive_smartfail_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_drives_drive_stopfail_item(
        &self,
        drives_drive_stopfail_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_drives_drive_suspend_item(
        &self,
        drives_drive_suspend_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_node_reboot_item(
        &self,
        node_reboot_item: crate::models::Empty,
        lnn: i32,
        force: bool,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn create_node_shutdown_item(
        &self,
        node_shutdown_item: crate::models::Empty,
        lnn: i32,
        force: bool,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>>;
    fn get_drives_drive_firmware(
        &self,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::DrivesDriveFirmware, Error = Error>>;
    fn get_node_drive(
        &self,
        node_drive_id: &str,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeDrives, Error = Error>>;
    fn get_node_driveconfig(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeDriveconfig, Error = Error>>;
    fn get_node_drives(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeDrives, Error = Error>>;
    fn get_node_drives_purposelist(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeDrivesPurposelist, Error = Error>>;
    fn get_node_hardware(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeHardware, Error = Error>>;
    fn get_node_hardware_fast(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeHardwareFast, Error = Error>>;
    fn get_node_partitions(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodePartitions, Error = Error>>;
    fn get_node_sensors(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeSensors, Error = Error>>;
    fn get_node_sled(
        &self,
        node_sled_id: &str,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeSleds, Error = Error>>;
    fn get_node_sleds(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeSleds, Error = Error>>;
    fn get_node_state(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeState, Error = Error>>;
    fn get_node_state_readonly(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStateReadonly, Error = Error>>;
    fn get_node_state_servicelight(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStateServicelight, Error = Error>>;
    fn get_node_state_smartfail(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStateSmartfail, Error = Error>>;
    fn get_node_status(
        &self,
        lnn: Option<i32>,
    ) -> Box<dyn Future<Item = crate::models::NodeStatus, Error = Error>>;
    fn get_node_status_batterystatus(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStatusBatterystatus, Error = Error>>;
    fn list_drives_drive_firmware_update(
        &self,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::DrivesDriveFirmwareUpdate, Error = Error>>;
    fn update_node_driveconfig(
        &self,
        node_driveconfig: crate::models::NodeDriveconfigExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_node_state_readonly(
        &self,
        node_state_readonly: crate::models::NodeStateReadonlyExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_node_state_servicelight(
        &self,
        node_state_servicelight: crate::models::NodeStateServicelightExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn update_node_state_smartfail(
        &self,
        node_state_smartfail: crate::models::NodeStateSmartfailExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> ClusterNodesApi for ClusterNodesApiClient<C> {
    fn create_drives_drive_add_item(
        &self,
        drives_drive_add_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/add",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &drives_drive_add_item,
            hyper::Method::POST,
        )
    }

    fn create_drives_drive_firmware_update_item(
        &self,
        drives_drive_firmware_update_item: crate::models::DrivesDriveFirmwareUpdateItem,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/firmware/update",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &drives_drive_firmware_update_item,
            hyper::Method::POST,
        )
    }

    fn create_drives_drive_format_item(
        &self,
        drives_drive_format_item: crate::models::DrivesDriveFormatItem,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/format",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &drives_drive_format_item,
            hyper::Method::POST,
        )
    }

    fn create_drives_drive_purpose_item(
        &self,
        drives_drive_purpose_item: crate::models::DrivesDrivePurposeItem,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/purpose",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &drives_drive_purpose_item,
            hyper::Method::POST,
        )
    }

    fn create_drives_drive_smartfail_item(
        &self,
        drives_drive_smartfail_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/smartfail",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &drives_drive_smartfail_item,
            hyper::Method::POST,
        )
    }

    fn create_drives_drive_stopfail_item(
        &self,
        drives_drive_stopfail_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/stopfail",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &drives_drive_stopfail_item,
            hyper::Method::POST,
        )
    }

    fn create_drives_drive_suspend_item(
        &self,
        drives_drive_suspend_item: crate::models::Empty,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/suspend",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &drives_drive_suspend_item,
            hyper::Method::POST,
        )
    }

    fn create_node_reboot_item(
        &self,
        node_reboot_item: crate::models::Empty,
        lnn: i32,
        force: bool,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("force", &force.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/reboot?{}",
            self.configuration.base_path,
            q,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &node_reboot_item,
            hyper::Method::POST,
        )
    }

    fn create_node_shutdown_item(
        &self,
        node_shutdown_item: crate::models::Empty,
        lnn: i32,
        force: bool,
    ) -> Box<dyn Future<Item = crate::models::Empty, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("force", &force.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/shutdown?{}",
            self.configuration.base_path,
            q,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &node_shutdown_item,
            hyper::Method::POST,
        )
    }

    fn get_drives_drive_firmware(
        &self,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::DrivesDriveFirmware, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/firmware",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_drive(
        &self,
        node_drive_id: &str,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeDrives, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/drives/{NodeDriveId}?{}",
            self.configuration.base_path,
            q,
            NodeDriveId = node_drive_id,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_driveconfig(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeDriveconfig, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/driveconfig?{}",
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

    fn get_node_drives(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeDrives, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/drives?{}",
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

    fn get_node_drives_purposelist(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeDrivesPurposelist, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives-purposelist",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_hardware(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeHardware, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/hardware?{}",
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

    fn get_node_hardware_fast(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeHardwareFast, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/hardware-fast",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_partitions(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodePartitions, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/partitions",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_sensors(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeSensors, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/sensors",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_sled(
        &self,
        node_sled_id: &str,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeSleds, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/sleds/{NodeSledId}?{}",
            self.configuration.base_path,
            q,
            NodeSledId = node_sled_id,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_sleds(
        &self,
        lnn: i32,
        timeout: f32,
    ) -> Box<dyn Future<Item = crate::models::NodeSleds, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout", &timeout.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/sleds?{}",
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

    fn get_node_state(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeState, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/state",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_state_readonly(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStateReadonly, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/state/readonly",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_state_servicelight(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStateServicelight, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/state/servicelight",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_state_smartfail(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStateSmartfail, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/state/smartfail",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_status(
        &self,
        lnn: Option<i32>,
    ) -> Box<dyn Future<Item = crate::models::NodeStatus, Error = Error>> {
        let uri_str = match lnn {
            Some(lnn) => format!(
                "{}/platform/3/cluster/nodes/{Lnn}/status",
                self.configuration.base_path,
                Lnn = lnn
            ),
            None => format!(
                "{}/platform/3/cluster/nodes/All/status",
                self.configuration.base_path,
            ),
        };

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_node_status_batterystatus(
        &self,
        lnn: i32,
    ) -> Box<dyn Future<Item = crate::models::NodeStatusBatterystatus, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/status/batterystatus",
            self.configuration.base_path,
            Lnn = lnn
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_drives_drive_firmware_update(
        &self,
        lnn: i32,
        driveid: &str,
    ) -> Box<dyn Future<Item = crate::models::DrivesDriveFirmwareUpdate, Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/firmware/update",
            self.configuration.base_path,
            Lnn = lnn,
            Driveid = driveid
        );

        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_node_driveconfig(
        &self,
        node_driveconfig: crate::models::NodeDriveconfigExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/5/cluster/nodes/{Lnn}/driveconfig",
            self.configuration.base_path,
            Lnn = lnn
        );

        put(self.configuration.borrow(), &uri_str, &node_driveconfig)
    }

    fn update_node_state_readonly(
        &self,
        node_state_readonly: crate::models::NodeStateReadonlyExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/state/readonly",
            self.configuration.base_path,
            Lnn = lnn
        );

        put(self.configuration.borrow(), &uri_str, &node_state_readonly)
    }

    fn update_node_state_servicelight(
        &self,
        node_state_servicelight: crate::models::NodeStateServicelightExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/state/servicelight",
            self.configuration.base_path,
            Lnn = lnn
        );

        put(
            self.configuration.borrow(),
            &uri_str,
            &node_state_servicelight,
        )
    }

    fn update_node_state_smartfail(
        &self,
        node_state_smartfail: crate::models::NodeStateSmartfailExtended,
        lnn: i32,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/3/cluster/nodes/{Lnn}/state/smartfail",
            self.configuration.base_path,
            Lnn = lnn
        );

        put(self.configuration.borrow(), &uri_str, &node_state_smartfail)
    }
}
