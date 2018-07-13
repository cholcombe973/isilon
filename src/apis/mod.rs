use cookie;
use hyper;
use reqwest;
use serde_json;

use std::error::Error as err;
use std::io;

#[derive(Debug)]
pub enum Error {
    E(String),
    Cookie(cookie::ParseError),
    Hyper(hyper::Error),
    Io(io::Error),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    /// Session token needs to be recreated.  Call Configuration::login
    SessionExpired,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.description())
    }
}

impl err for Error {
    fn description(&self) -> &str {
        match *self {
            Error::E(ref s) => s,
            Error::Cookie(ref e) => e.description(),
            Error::Hyper(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Reqwest(ref e) => e.description(),
            Error::Serde(ref e) => e.description(),
            Error::SessionExpired => "Session Expired",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::E(_) => None,
            Error::Cookie(ref e) => e.cause(),
            Error::Hyper(ref e) => e.cause(),
            Error::Io(ref e) => e.cause(),
            Error::Reqwest(ref e) => e.cause(),
            Error::Serde(ref e) => e.cause(),
            Error::SessionExpired => None,
        }
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        return Error::E(e);
    }
}

impl From<cookie::ParseError> for Error {
    fn from(e: cookie::ParseError) -> Self {
        return Error::Cookie(e);
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e);
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        return Error::Io(e);
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::Reqwest(e);
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

use super::models::*;

mod antivirus_api;
pub use self::antivirus_api::{AntivirusApi, AntivirusApiClient};
mod audit_api;
pub use self::audit_api::{AuditApi, AuditApiClient};
mod auth_api;
pub use self::auth_api::{AuthApi, AuthApiClient};
mod auth_groups_api;
pub use self::auth_groups_api::{AuthGroupsApi, AuthGroupsApiClient};
mod auth_providers_api;
pub use self::auth_providers_api::{AuthProvidersApi, AuthProvidersApiClient};
mod auth_roles_api;
pub use self::auth_roles_api::{AuthRolesApi, AuthRolesApiClient};
mod auth_users_api;
pub use self::auth_users_api::{AuthUsersApi, AuthUsersApiClient};
mod certificate_api;
pub use self::certificate_api::{CertificateApi, CertificateApiClient};
mod cloud_api;
pub use self::cloud_api::{CloudApi, CloudApiClient};
mod cluster_api;
pub use self::cluster_api::{ClusterApi, ClusterApiClient};
mod cluster_nodes_api;
pub use self::cluster_nodes_api::{ClusterNodesApi, ClusterNodesApiClient};
mod debug_api;
pub use self::debug_api::{DebugApi, DebugApiClient};
mod dedupe_api;
pub use self::dedupe_api::{DedupeApi, DedupeApiClient};
mod event_api;
pub use self::event_api::{EventApi, EventApiClient};
mod file_filter_api;
pub use self::file_filter_api::{FileFilterApi, FileFilterApiClient};
mod filepool_api;
pub use self::filepool_api::{FilepoolApi, FilepoolApiClient};
mod filesystem_api;
pub use self::filesystem_api::{FilesystemApi, FilesystemApiClient};
mod fsa_api;
pub use self::fsa_api::{FsaApi, FsaApiClient};
mod fsa_results_api;
pub use self::fsa_results_api::{FsaResultsApi, FsaResultsApiClient};
mod hardening_api;
pub use self::hardening_api::{HardeningApi, HardeningApiClient};
mod hardware_api;
pub use self::hardware_api::{HardwareApi, HardwareApiClient};
mod id_resolution_api;
pub use self::id_resolution_api::{IdResolutionApi, IdResolutionApiClient};
mod job_api;
pub use self::job_api::{JobApi, JobApiClient};
mod license_api;
pub use self::license_api::{LicenseApi, LicenseApiClient};
mod local_api;
pub use self::local_api::{LocalApi, LocalApiClient};
mod namespace_api;
pub use self::namespace_api::{NamespaceApi, NamespaceApiClient};
mod network_api;
pub use self::network_api::{NetworkApi, NetworkApiClient};
mod network_groupnets_api;
pub use self::network_groupnets_api::{NetworkGroupnetsApi, NetworkGroupnetsApiClient};
mod network_groupnets_subnets_api;
pub use self::network_groupnets_subnets_api::{
    NetworkGroupnetsSubnetsApi, NetworkGroupnetsSubnetsApiClient,
};
mod protocols_api;
pub use self::protocols_api::{ProtocolsApi, ProtocolsApiClient};
mod protocols_hdfs_api;
pub use self::protocols_hdfs_api::{ProtocolsHdfsApi, ProtocolsHdfsApiClient};
mod quota_api;
pub use self::quota_api::{QuotaApi, QuotaApiClient};
mod quota_quotas_api;
pub use self::quota_quotas_api::{QuotaQuotasApi, QuotaQuotasApiClient};
mod quota_reports_api;
pub use self::quota_reports_api::{QuotaReportsApi, QuotaReportsApiClient};
mod remotesupport_api;
pub use self::remotesupport_api::{RemotesupportApi, RemotesupportApiClient};
mod snapshot_api;
pub use self::snapshot_api::{SnapshotApi, SnapshotApiClient};
mod snapshot_changelists_api;
pub use self::snapshot_changelists_api::{SnapshotChangelistsApi, SnapshotChangelistsApiClient};
mod snapshot_snapshots_api;
pub use self::snapshot_snapshots_api::{SnapshotSnapshotsApi, SnapshotSnapshotsApiClient};
mod statistics_api;
pub use self::statistics_api::{StatisticsApi, StatisticsApiClient};
mod storagepool_api;
pub use self::storagepool_api::{StoragepoolApi, StoragepoolApiClient};
mod sync_api;
pub use self::sync_api::{SyncApi, SyncApiClient};
mod sync_policies_api;
pub use self::sync_policies_api::{SyncPoliciesApi, SyncPoliciesApiClient};
mod sync_reports_api;
pub use self::sync_reports_api::{SyncReportsApi, SyncReportsApiClient};
mod sync_target_api;
pub use self::sync_target_api::{SyncTargetApi, SyncTargetApiClient};
mod upgrade_api;
pub use self::upgrade_api::{UpgradeApi, UpgradeApiClient};
mod upgrade_cluster_api;
pub use self::upgrade_cluster_api::{UpgradeClusterApi, UpgradeClusterApiClient};
mod worm_api;
pub use self::worm_api::{WormApi, WormApiClient};
mod zones_api;
pub use self::zones_api::{ZonesApi, ZonesApiClient};
mod zones_summary_api;
pub use self::zones_summary_api::{ZonesSummaryApi, ZonesSummaryApiClient};

pub mod client;
pub mod configuration;
