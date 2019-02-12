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

pub struct SnapshotSnapshotsApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> SnapshotSnapshotsApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> SnapshotSnapshotsApiClient<C> {
        SnapshotSnapshotsApiClient {
            configuration: configuration,
        }
    }
}

pub trait SnapshotSnapshotsApi {
    fn create_snapshot_lock(
        &self,
        snapshot_lock: crate::models::SnapshotLockCreateParams,
        sid: &str,
    ) -> Box<dyn Future<Item = crate::models::CreateSnapshotLockResponse, Error = Error>>;
    fn delete_snapshot_lock(
        &self,
        snapshot_lock_id: &str,
        sid: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
    fn delete_snapshot_locks(&self, sid: &str) -> Box<dyn Future<Item = (), Error = Error>>;
    fn get_snapshot_lock(
        &self,
        snapshot_lock_id: &str,
        sid: &str,
    ) -> Box<dyn Future<Item = crate::models::SnapshotLocks, Error = Error>>;
    fn list_snapshot_locks(
        &self,
        sid: &str,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Box<dyn Future<Item = crate::models::SnapshotLocksExtended, Error = Error>>;
    fn update_snapshot_lock(
        &self,
        snapshot_lock: crate::models::SnapshotLock,
        snapshot_lock_id: &str,
        sid: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>>;
}

impl<C: hyper::client::connect::Connect + 'static> SnapshotSnapshotsApi
    for SnapshotSnapshotsApiClient<C>
{
    fn create_snapshot_lock(
        &self,
        snapshot_lock: crate::models::SnapshotLockCreateParams,
        sid: &str,
    ) -> Box<dyn Future<Item = crate::models::CreateSnapshotLockResponse, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/snapshot/snapshots/{Sid}/locks",
            self.configuration.base_path,
            Sid = sid
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &snapshot_lock,
            hyper::Method::POST,
        )
    }

    fn delete_snapshot_lock(
        &self,
        snapshot_lock_id: &str,
        sid: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/snapshot/snapshots/{Sid}/locks/{SnapshotLockId}",
            self.configuration.base_path,
            SnapshotLockId = snapshot_lock_id,
            Sid = sid
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn delete_snapshot_locks(&self, sid: &str) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/snapshot/snapshots/{Sid}/locks",
            self.configuration.base_path,
            Sid = sid
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn get_snapshot_lock(
        &self,
        snapshot_lock_id: &str,
        sid: &str,
    ) -> Box<dyn Future<Item = crate::models::SnapshotLocks, Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/snapshot/snapshots/{Sid}/locks/{SnapshotLockId}",
            self.configuration.base_path,
            SnapshotLockId = snapshot_lock_id,
            Sid = sid
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_snapshot_locks(
        &self,
        sid: &str,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Box<dyn Future<Item = crate::models::SnapshotLocksExtended, Error = Error>> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .append_pair("resume", &resume.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/snapshot/snapshots/{Sid}/locks?{}",
            self.configuration.base_path,
            q,
            Sid = sid
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_snapshot_lock(
        &self,
        snapshot_lock: crate::models::SnapshotLock,
        snapshot_lock_id: &str,
        sid: &str,
    ) -> Box<dyn Future<Item = (), Error = Error>> {
        let uri_str = format!(
            "{}/platform/1/snapshot/snapshots/{Sid}/locks/{SnapshotLockId}",
            self.configuration.base_path,
            SnapshotLockId = snapshot_lock_id,
            Sid = sid
        );
        put(self.configuration.borrow(), &uri_str, &snapshot_lock)
    }
}
