use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient<C: hyper::client::Connect> {
    configuration: Rc<Configuration<C>>,
    antivirus_api: Box<::apis::AntivirusApi>,
    audit_api: Box<::apis::AuditApi>,
    auth_api: Box<::apis::AuthApi>,
    auth_groups_api: Box<::apis::AuthGroupsApi>,
    auth_providers_api: Box<::apis::AuthProvidersApi>,
    auth_roles_api: Box<::apis::AuthRolesApi>,
    auth_users_api: Box<::apis::AuthUsersApi>,
    certificate_api: Box<::apis::CertificateApi>,
    cloud_api: Box<::apis::CloudApi>,
    cluster_api: Box<::apis::ClusterApi>,
    cluster_nodes_api: Box<::apis::ClusterNodesApi>,
    debug_api: Box<::apis::DebugApi>,
    dedupe_api: Box<::apis::DedupeApi>,
    event_api: Box<::apis::EventApi>,
    file_filter_api: Box<::apis::FileFilterApi>,
    filepool_api: Box<::apis::FilepoolApi>,
    filesystem_api: Box<::apis::FilesystemApi>,
    fsa_api: Box<::apis::FsaApi>,
    fsa_results_api: Box<::apis::FsaResultsApi>,
    hardening_api: Box<::apis::HardeningApi>,
    hardware_api: Box<::apis::HardwareApi>,
    id_resolution_api: Box<::apis::IdResolutionApi>,
    job_api: Box<::apis::JobApi>,
    license_api: Box<::apis::LicenseApi>,
    local_api: Box<::apis::LocalApi>,
    namespace_api: Box<::apis::NamespaceApi>,
    network_api: Box<::apis::NetworkApi>,
    network_groupnets_api: Box<::apis::NetworkGroupnetsApi>,
    network_groupnets_subnets_api: Box<::apis::NetworkGroupnetsSubnetsApi>,
    protocols_api: Box<::apis::ProtocolsApi>,
    protocols_hdfs_api: Box<::apis::ProtocolsHdfsApi>,
    quota_api: Box<::apis::QuotaApi>,
    quota_quotas_api: Box<::apis::QuotaQuotasApi>,
    quota_reports_api: Box<::apis::QuotaReportsApi>,
    remotesupport_api: Box<::apis::RemotesupportApi>,
    snapshot_api: Box<::apis::SnapshotApi>,
    snapshot_changelists_api: Box<::apis::SnapshotChangelistsApi>,
    snapshot_snapshots_api: Box<::apis::SnapshotSnapshotsApi>,
    statistics_api: Box<::apis::StatisticsApi>,
    storagepool_api: Box<::apis::StoragepoolApi>,
    sync_api: Box<::apis::SyncApi>,
    sync_policies_api: Box<::apis::SyncPoliciesApi>,
    sync_reports_api: Box<::apis::SyncReportsApi>,
    sync_target_api: Box<::apis::SyncTargetApi>,
    upgrade_api: Box<::apis::UpgradeApi>,
    upgrade_cluster_api: Box<::apis::UpgradeClusterApi>,
    worm_api: Box<::apis::WormApi>,
    zones_api: Box<::apis::ZonesApi>,
    zones_summary_api: Box<::apis::ZonesSummaryApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            antivirus_api: Box::new(::apis::AntivirusApiClient::new(rc.clone())),
            audit_api: Box::new(::apis::AuditApiClient::new(rc.clone())),
            auth_api: Box::new(::apis::AuthApiClient::new(rc.clone())),
            auth_groups_api: Box::new(::apis::AuthGroupsApiClient::new(rc.clone())),
            auth_providers_api: Box::new(::apis::AuthProvidersApiClient::new(rc.clone())),
            auth_roles_api: Box::new(::apis::AuthRolesApiClient::new(rc.clone())),
            auth_users_api: Box::new(::apis::AuthUsersApiClient::new(rc.clone())),
            certificate_api: Box::new(::apis::CertificateApiClient::new(rc.clone())),
            cloud_api: Box::new(::apis::CloudApiClient::new(rc.clone())),
            cluster_api: Box::new(::apis::ClusterApiClient::new(rc.clone())),
            cluster_nodes_api: Box::new(::apis::ClusterNodesApiClient::new(rc.clone())),
            debug_api: Box::new(::apis::DebugApiClient::new(rc.clone())),
            dedupe_api: Box::new(::apis::DedupeApiClient::new(rc.clone())),
            event_api: Box::new(::apis::EventApiClient::new(rc.clone())),
            file_filter_api: Box::new(::apis::FileFilterApiClient::new(rc.clone())),
            filepool_api: Box::new(::apis::FilepoolApiClient::new(rc.clone())),
            filesystem_api: Box::new(::apis::FilesystemApiClient::new(rc.clone())),
            fsa_api: Box::new(::apis::FsaApiClient::new(rc.clone())),
            fsa_results_api: Box::new(::apis::FsaResultsApiClient::new(rc.clone())),
            hardening_api: Box::new(::apis::HardeningApiClient::new(rc.clone())),
            hardware_api: Box::new(::apis::HardwareApiClient::new(rc.clone())),
            id_resolution_api: Box::new(::apis::IdResolutionApiClient::new(rc.clone())),
            job_api: Box::new(::apis::JobApiClient::new(rc.clone())),
            license_api: Box::new(::apis::LicenseApiClient::new(rc.clone())),
            local_api: Box::new(::apis::LocalApiClient::new(rc.clone())),
            namespace_api: Box::new(::apis::NamespaceApiClient::new(rc.clone())),
            network_api: Box::new(::apis::NetworkApiClient::new(rc.clone())),
            network_groupnets_api: Box::new(::apis::NetworkGroupnetsApiClient::new(rc.clone())),
            network_groupnets_subnets_api: Box::new(::apis::NetworkGroupnetsSubnetsApiClient::new(
                rc.clone(),
            )),
            protocols_api: Box::new(::apis::ProtocolsApiClient::new(rc.clone())),
            protocols_hdfs_api: Box::new(::apis::ProtocolsHdfsApiClient::new(rc.clone())),
            quota_api: Box::new(::apis::QuotaApiClient::new(rc.clone())),
            quota_quotas_api: Box::new(::apis::QuotaQuotasApiClient::new(rc.clone())),
            quota_reports_api: Box::new(::apis::QuotaReportsApiClient::new(rc.clone())),
            remotesupport_api: Box::new(::apis::RemotesupportApiClient::new(rc.clone())),
            snapshot_api: Box::new(::apis::SnapshotApiClient::new(rc.clone())),
            snapshot_changelists_api: Box::new(::apis::SnapshotChangelistsApiClient::new(
                rc.clone(),
            )),
            snapshot_snapshots_api: Box::new(::apis::SnapshotSnapshotsApiClient::new(rc.clone())),
            statistics_api: Box::new(::apis::StatisticsApiClient::new(rc.clone())),
            storagepool_api: Box::new(::apis::StoragepoolApiClient::new(rc.clone())),
            sync_api: Box::new(::apis::SyncApiClient::new(rc.clone())),
            sync_policies_api: Box::new(::apis::SyncPoliciesApiClient::new(rc.clone())),
            sync_reports_api: Box::new(::apis::SyncReportsApiClient::new(rc.clone())),
            sync_target_api: Box::new(::apis::SyncTargetApiClient::new(rc.clone())),
            upgrade_api: Box::new(::apis::UpgradeApiClient::new(rc.clone())),
            upgrade_cluster_api: Box::new(::apis::UpgradeClusterApiClient::new(rc.clone())),
            worm_api: Box::new(::apis::WormApiClient::new(rc.clone())),
            zones_api: Box::new(::apis::ZonesApiClient::new(rc.clone())),
            zones_summary_api: Box::new(::apis::ZonesSummaryApiClient::new(rc.clone())),
        }
    }

    pub fn antivirus_api(&self) -> &::apis::AntivirusApi {
        self.antivirus_api.as_ref()
    }

    pub fn audit_api(&self) -> &::apis::AuditApi {
        self.audit_api.as_ref()
    }

    pub fn auth_api(&self) -> &::apis::AuthApi {
        self.auth_api.as_ref()
    }

    pub fn auth_groups_api(&self) -> &::apis::AuthGroupsApi {
        self.auth_groups_api.as_ref()
    }

    pub fn auth_providers_api(&self) -> &::apis::AuthProvidersApi {
        self.auth_providers_api.as_ref()
    }

    pub fn auth_roles_api(&self) -> &::apis::AuthRolesApi {
        self.auth_roles_api.as_ref()
    }

    pub fn auth_users_api(&self) -> &::apis::AuthUsersApi {
        self.auth_users_api.as_ref()
    }

    pub fn certificate_api(&self) -> &::apis::CertificateApi {
        self.certificate_api.as_ref()
    }

    pub fn cloud_api(&self) -> &::apis::CloudApi {
        self.cloud_api.as_ref()
    }

    pub fn cluster_api(&self) -> &::apis::ClusterApi {
        self.cluster_api.as_ref()
    }

    pub fn cluster_nodes_api(&self) -> &::apis::ClusterNodesApi {
        self.cluster_nodes_api.as_ref()
    }

    pub fn debug_api(&self) -> &::apis::DebugApi {
        self.debug_api.as_ref()
    }

    pub fn dedupe_api(&self) -> &::apis::DedupeApi {
        self.dedupe_api.as_ref()
    }

    pub fn event_api(&self) -> &::apis::EventApi {
        self.event_api.as_ref()
    }

    pub fn file_filter_api(&self) -> &::apis::FileFilterApi {
        self.file_filter_api.as_ref()
    }

    pub fn filepool_api(&self) -> &::apis::FilepoolApi {
        self.filepool_api.as_ref()
    }

    pub fn filesystem_api(&self) -> &::apis::FilesystemApi {
        self.filesystem_api.as_ref()
    }

    pub fn fsa_api(&self) -> &::apis::FsaApi {
        self.fsa_api.as_ref()
    }

    pub fn fsa_results_api(&self) -> &::apis::FsaResultsApi {
        self.fsa_results_api.as_ref()
    }

    pub fn hardening_api(&self) -> &::apis::HardeningApi {
        self.hardening_api.as_ref()
    }

    pub fn hardware_api(&self) -> &::apis::HardwareApi {
        self.hardware_api.as_ref()
    }

    pub fn id_resolution_api(&self) -> &::apis::IdResolutionApi {
        self.id_resolution_api.as_ref()
    }

    pub fn job_api(&self) -> &::apis::JobApi {
        self.job_api.as_ref()
    }

    pub fn license_api(&self) -> &::apis::LicenseApi {
        self.license_api.as_ref()
    }

    pub fn local_api(&self) -> &::apis::LocalApi {
        self.local_api.as_ref()
    }

    pub fn namespace_api(&self) -> &::apis::NamespaceApi {
        self.namespace_api.as_ref()
    }

    pub fn network_api(&self) -> &::apis::NetworkApi {
        self.network_api.as_ref()
    }

    pub fn network_groupnets_api(&self) -> &::apis::NetworkGroupnetsApi {
        self.network_groupnets_api.as_ref()
    }

    pub fn network_groupnets_subnets_api(&self) -> &::apis::NetworkGroupnetsSubnetsApi {
        self.network_groupnets_subnets_api.as_ref()
    }

    pub fn protocols_api(&self) -> &::apis::ProtocolsApi {
        self.protocols_api.as_ref()
    }

    pub fn protocols_hdfs_api(&self) -> &::apis::ProtocolsHdfsApi {
        self.protocols_hdfs_api.as_ref()
    }

    pub fn quota_api(&self) -> &::apis::QuotaApi {
        self.quota_api.as_ref()
    }

    pub fn quota_quotas_api(&self) -> &::apis::QuotaQuotasApi {
        self.quota_quotas_api.as_ref()
    }

    pub fn quota_reports_api(&self) -> &::apis::QuotaReportsApi {
        self.quota_reports_api.as_ref()
    }

    pub fn remotesupport_api(&self) -> &::apis::RemotesupportApi {
        self.remotesupport_api.as_ref()
    }

    pub fn snapshot_api(&self) -> &::apis::SnapshotApi {
        self.snapshot_api.as_ref()
    }

    pub fn snapshot_changelists_api(&self) -> &::apis::SnapshotChangelistsApi {
        self.snapshot_changelists_api.as_ref()
    }

    pub fn snapshot_snapshots_api(&self) -> &::apis::SnapshotSnapshotsApi {
        self.snapshot_snapshots_api.as_ref()
    }

    pub fn statistics_api(&self) -> &::apis::StatisticsApi {
        self.statistics_api.as_ref()
    }

    pub fn storagepool_api(&self) -> &::apis::StoragepoolApi {
        self.storagepool_api.as_ref()
    }

    pub fn sync_api(&self) -> &::apis::SyncApi {
        self.sync_api.as_ref()
    }

    pub fn sync_policies_api(&self) -> &::apis::SyncPoliciesApi {
        self.sync_policies_api.as_ref()
    }

    pub fn sync_reports_api(&self) -> &::apis::SyncReportsApi {
        self.sync_reports_api.as_ref()
    }

    pub fn sync_target_api(&self) -> &::apis::SyncTargetApi {
        self.sync_target_api.as_ref()
    }

    pub fn upgrade_api(&self) -> &::apis::UpgradeApi {
        self.upgrade_api.as_ref()
    }

    pub fn upgrade_cluster_api(&self) -> &::apis::UpgradeClusterApi {
        self.upgrade_cluster_api.as_ref()
    }

    pub fn worm_api(&self) -> &::apis::WormApi {
        self.worm_api.as_ref()
    }

    pub fn zones_api(&self) -> &::apis::ZonesApi {
        self.zones_api.as_ref()
    }

    pub fn zones_summary_api(&self) -> &::apis::ZonesSummaryApi {
        self.zones_summary_api.as_ref()
    }
}
