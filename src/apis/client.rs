use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient<C: hyper::client::connect::Connect> {
    configuration: Rc<Configuration<C>>,
    antivirus_api: Box<dyn crate::apis::AntivirusApi>,
    audit_api: Box<dyn crate::apis::AuditApi>,
    auth_api: Box<dyn crate::apis::AuthApi>,
    auth_groups_api: Box<dyn crate::apis::AuthGroupsApi>,
    auth_providers_api: Box<dyn crate::apis::AuthProvidersApi>,
    auth_roles_api: Box<dyn crate::apis::AuthRolesApi>,
    auth_users_api: Box<dyn crate::apis::AuthUsersApi>,
    certificate_api: Box<dyn crate::apis::CertificateApi>,
    cloud_api: Box<dyn crate::apis::CloudApi>,
    cluster_api: Box<dyn crate::apis::ClusterApi>,
    cluster_nodes_api: Box<dyn crate::apis::ClusterNodesApi>,
    debug_api: Box<dyn crate::apis::DebugApi>,
    dedupe_api: Box<dyn crate::apis::DedupeApi>,
    event_api: Box<dyn crate::apis::EventApi>,
    file_filter_api: Box<dyn crate::apis::FileFilterApi>,
    filepool_api: Box<dyn crate::apis::FilepoolApi>,
    filesystem_api: Box<dyn crate::apis::FilesystemApi>,
    fsa_api: Box<dyn crate::apis::FsaApi>,
    fsa_results_api: Box<dyn crate::apis::FsaResultsApi>,
    hardening_api: Box<dyn crate::apis::HardeningApi>,
    hardware_api: Box<dyn crate::apis::HardwareApi>,
    id_resolution_api: Box<dyn crate::apis::IdResolutionApi>,
    job_api: Box<dyn crate::apis::JobApi>,
    license_api: Box<dyn crate::apis::LicenseApi>,
    local_api: Box<dyn crate::apis::LocalApi>,
    namespace_api: Box<dyn crate::apis::NamespaceApi>,
    network_api: Box<dyn crate::apis::NetworkApi>,
    network_groupnets_api: Box<dyn crate::apis::NetworkGroupnetsApi>,
    network_groupnets_subnets_api: Box<dyn crate::apis::NetworkGroupnetsSubnetsApi>,
    protocols_api: Box<dyn crate::apis::ProtocolsApi>,
    protocols_hdfs_api: Box<dyn crate::apis::ProtocolsHdfsApi>,
    quota_api: Box<dyn crate::apis::QuotaApi>,
    quota_quotas_api: Box<dyn crate::apis::QuotaQuotasApi>,
    quota_reports_api: Box<dyn crate::apis::QuotaReportsApi>,
    remotesupport_api: Box<dyn crate::apis::RemotesupportApi>,
    snapshot_api: Box<dyn crate::apis::SnapshotApi>,
    snapshot_changelists_api: Box<dyn crate::apis::SnapshotChangelistsApi>,
    snapshot_snapshots_api: Box<dyn crate::apis::SnapshotSnapshotsApi>,
    statistics_api: Box<dyn crate::apis::StatisticsApi>,
    storagepool_api: Box<dyn crate::apis::StoragepoolApi>,
    sync_api: Box<dyn crate::apis::SyncApi>,
    sync_policies_api: Box<dyn crate::apis::SyncPoliciesApi>,
    sync_reports_api: Box<dyn crate::apis::SyncReportsApi>,
    sync_target_api: Box<dyn crate::apis::SyncTargetApi>,
    upgrade_api: Box<dyn crate::apis::UpgradeApi>,
    upgrade_cluster_api: Box<dyn crate::apis::UpgradeClusterApi>,
    worm_api: Box<dyn crate::apis::WormApi>,
    zones_api: Box<dyn crate::apis::ZonesApi>,
    zones_summary_api: Box<dyn crate::apis::ZonesSummaryApi>,
}

impl<C: hyper::client::connect::Connect + 'static + std::marker::Sync + std::marker::Send + Clone> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            antivirus_api: Box::new(crate::apis::AntivirusApiClient::new(rc.clone())),
            audit_api: Box::new(crate::apis::AuditApiClient::new(rc.clone())),
            auth_api: Box::new(crate::apis::AuthApiClient::new(rc.clone())),
            auth_groups_api: Box::new(crate::apis::AuthGroupsApiClient::new(rc.clone())),
            auth_providers_api: Box::new(crate::apis::AuthProvidersApiClient::new(rc.clone())),
            auth_roles_api: Box::new(crate::apis::AuthRolesApiClient::new(rc.clone())),
            auth_users_api: Box::new(crate::apis::AuthUsersApiClient::new(rc.clone())),
            certificate_api: Box::new(crate::apis::CertificateApiClient::new(rc.clone())),
            cloud_api: Box::new(crate::apis::CloudApiClient::new(rc.clone())),
            cluster_api: Box::new(crate::apis::ClusterApiClient::new(rc.clone())),
            cluster_nodes_api: Box::new(crate::apis::ClusterNodesApiClient::new(rc.clone())),
            debug_api: Box::new(crate::apis::DebugApiClient::new(rc.clone())),
            dedupe_api: Box::new(crate::apis::DedupeApiClient::new(rc.clone())),
            event_api: Box::new(crate::apis::EventApiClient::new(rc.clone())),
            file_filter_api: Box::new(crate::apis::FileFilterApiClient::new(rc.clone())),
            filepool_api: Box::new(crate::apis::FilepoolApiClient::new(rc.clone())),
            filesystem_api: Box::new(crate::apis::FilesystemApiClient::new(rc.clone())),
            fsa_api: Box::new(crate::apis::FsaApiClient::new(rc.clone())),
            fsa_results_api: Box::new(crate::apis::FsaResultsApiClient::new(rc.clone())),
            hardening_api: Box::new(crate::apis::HardeningApiClient::new(rc.clone())),
            hardware_api: Box::new(crate::apis::HardwareApiClient::new(rc.clone())),
            id_resolution_api: Box::new(crate::apis::IdResolutionApiClient::new(rc.clone())),
            job_api: Box::new(crate::apis::JobApiClient::new(rc.clone())),
            license_api: Box::new(crate::apis::LicenseApiClient::new(rc.clone())),
            local_api: Box::new(crate::apis::LocalApiClient::new(rc.clone())),
            namespace_api: Box::new(crate::apis::NamespaceApiClient::new(rc.clone())),
            network_api: Box::new(crate::apis::NetworkApiClient::new(rc.clone())),
            network_groupnets_api: Box::new(crate::apis::NetworkGroupnetsApiClient::new(
                rc.clone(),
            )),
            network_groupnets_subnets_api: Box::new(
                crate::apis::NetworkGroupnetsSubnetsApiClient::new(rc.clone()),
            ),
            protocols_api: Box::new(crate::apis::ProtocolsApiClient::new(rc.clone())),
            protocols_hdfs_api: Box::new(crate::apis::ProtocolsHdfsApiClient::new(rc.clone())),
            quota_api: Box::new(crate::apis::QuotaApiClient::new(rc.clone())),
            quota_quotas_api: Box::new(crate::apis::QuotaQuotasApiClient::new(rc.clone())),
            quota_reports_api: Box::new(crate::apis::QuotaReportsApiClient::new(rc.clone())),
            remotesupport_api: Box::new(crate::apis::RemotesupportApiClient::new(rc.clone())),
            snapshot_api: Box::new(crate::apis::SnapshotApiClient::new(rc.clone())),
            snapshot_changelists_api: Box::new(crate::apis::SnapshotChangelistsApiClient::new(
                rc.clone(),
            )),
            snapshot_snapshots_api: Box::new(crate::apis::SnapshotSnapshotsApiClient::new(
                rc.clone(),
            )),
            statistics_api: Box::new(crate::apis::StatisticsApiClient::new(rc.clone())),
            storagepool_api: Box::new(crate::apis::StoragepoolApiClient::new(rc.clone())),
            sync_api: Box::new(crate::apis::SyncApiClient::new(rc.clone())),
            sync_policies_api: Box::new(crate::apis::SyncPoliciesApiClient::new(rc.clone())),
            sync_reports_api: Box::new(crate::apis::SyncReportsApiClient::new(rc.clone())),
            sync_target_api: Box::new(crate::apis::SyncTargetApiClient::new(rc.clone())),
            upgrade_api: Box::new(crate::apis::UpgradeApiClient::new(rc.clone())),
            upgrade_cluster_api: Box::new(crate::apis::UpgradeClusterApiClient::new(rc.clone())),
            worm_api: Box::new(crate::apis::WormApiClient::new(rc.clone())),
            zones_api: Box::new(crate::apis::ZonesApiClient::new(rc.clone())),
            zones_summary_api: Box::new(crate::apis::ZonesSummaryApiClient::new(rc.clone())),
        }
    }

    pub fn antivirus_api(&self) -> &dyn crate::apis::AntivirusApi {
        self.antivirus_api.as_ref()
    }

    pub fn audit_api(&self) -> &dyn crate::apis::AuditApi {
        self.audit_api.as_ref()
    }

    pub fn auth_api(&self) -> &dyn crate::apis::AuthApi {
        self.auth_api.as_ref()
    }

    pub fn auth_groups_api(&self) -> &dyn crate::apis::AuthGroupsApi {
        self.auth_groups_api.as_ref()
    }

    pub fn auth_providers_api(&self) -> &dyn crate::apis::AuthProvidersApi {
        self.auth_providers_api.as_ref()
    }

    pub fn auth_roles_api(&self) -> &dyn crate::apis::AuthRolesApi {
        self.auth_roles_api.as_ref()
    }

    pub fn auth_users_api(&self) -> &dyn crate::apis::AuthUsersApi {
        self.auth_users_api.as_ref()
    }

    pub fn certificate_api(&self) -> &dyn crate::apis::CertificateApi {
        self.certificate_api.as_ref()
    }

    pub fn cloud_api(&self) -> &dyn crate::apis::CloudApi {
        self.cloud_api.as_ref()
    }

    pub fn cluster_api(&self) -> &dyn crate::apis::ClusterApi {
        self.cluster_api.as_ref()
    }

    pub fn cluster_nodes_api(&self) -> &dyn crate::apis::ClusterNodesApi {
        self.cluster_nodes_api.as_ref()
    }

    pub fn debug_api(&self) -> &dyn crate::apis::DebugApi {
        self.debug_api.as_ref()
    }

    pub fn dedupe_api(&self) -> &dyn crate::apis::DedupeApi {
        self.dedupe_api.as_ref()
    }

    pub fn event_api(&self) -> &dyn crate::apis::EventApi {
        self.event_api.as_ref()
    }

    pub fn file_filter_api(&self) -> &dyn crate::apis::FileFilterApi {
        self.file_filter_api.as_ref()
    }

    pub fn filepool_api(&self) -> &dyn crate::apis::FilepoolApi {
        self.filepool_api.as_ref()
    }

    pub fn filesystem_api(&self) -> &dyn crate::apis::FilesystemApi {
        self.filesystem_api.as_ref()
    }

    pub fn fsa_api(&self) -> &dyn crate::apis::FsaApi {
        self.fsa_api.as_ref()
    }

    pub fn fsa_results_api(&self) -> &dyn crate::apis::FsaResultsApi {
        self.fsa_results_api.as_ref()
    }

    pub fn hardening_api(&self) -> &dyn crate::apis::HardeningApi {
        self.hardening_api.as_ref()
    }

    pub fn hardware_api(&self) -> &dyn crate::apis::HardwareApi {
        self.hardware_api.as_ref()
    }

    pub fn id_resolution_api(&self) -> &dyn crate::apis::IdResolutionApi {
        self.id_resolution_api.as_ref()
    }

    pub fn job_api(&self) -> &dyn crate::apis::JobApi {
        self.job_api.as_ref()
    }

    pub fn license_api(&self) -> &dyn crate::apis::LicenseApi {
        self.license_api.as_ref()
    }

    pub fn local_api(&self) -> &dyn crate::apis::LocalApi {
        self.local_api.as_ref()
    }

    pub fn namespace_api(&self) -> &dyn crate::apis::NamespaceApi {
        self.namespace_api.as_ref()
    }

    pub fn network_api(&self) -> &dyn crate::apis::NetworkApi {
        self.network_api.as_ref()
    }

    pub fn network_groupnets_api(&self) -> &dyn crate::apis::NetworkGroupnetsApi {
        self.network_groupnets_api.as_ref()
    }

    pub fn network_groupnets_subnets_api(&self) -> &dyn crate::apis::NetworkGroupnetsSubnetsApi {
        self.network_groupnets_subnets_api.as_ref()
    }

    pub fn protocols_api(&self) -> &dyn crate::apis::ProtocolsApi {
        self.protocols_api.as_ref()
    }

    pub fn protocols_hdfs_api(&self) -> &dyn crate::apis::ProtocolsHdfsApi {
        self.protocols_hdfs_api.as_ref()
    }

    pub fn quota_api(&self) -> &dyn crate::apis::QuotaApi {
        self.quota_api.as_ref()
    }

    pub fn quota_quotas_api(&self) -> &dyn crate::apis::QuotaQuotasApi {
        self.quota_quotas_api.as_ref()
    }

    pub fn quota_reports_api(&self) -> &dyn crate::apis::QuotaReportsApi {
        self.quota_reports_api.as_ref()
    }

    pub fn remotesupport_api(&self) -> &dyn crate::apis::RemotesupportApi {
        self.remotesupport_api.as_ref()
    }

    pub fn snapshot_api(&self) -> &dyn crate::apis::SnapshotApi {
        self.snapshot_api.as_ref()
    }

    pub fn snapshot_changelists_api(&self) -> &dyn crate::apis::SnapshotChangelistsApi {
        self.snapshot_changelists_api.as_ref()
    }

    pub fn snapshot_snapshots_api(&self) -> &dyn crate::apis::SnapshotSnapshotsApi {
        self.snapshot_snapshots_api.as_ref()
    }

    pub fn statistics_api(&self) -> &dyn crate::apis::StatisticsApi {
        self.statistics_api.as_ref()
    }

    pub fn storagepool_api(&self) -> &dyn crate::apis::StoragepoolApi {
        self.storagepool_api.as_ref()
    }

    pub fn sync_api(&self) -> &dyn crate::apis::SyncApi {
        self.sync_api.as_ref()
    }

    pub fn sync_policies_api(&self) -> &dyn crate::apis::SyncPoliciesApi {
        self.sync_policies_api.as_ref()
    }

    pub fn sync_reports_api(&self) -> &dyn crate::apis::SyncReportsApi {
        self.sync_reports_api.as_ref()
    }

    pub fn sync_target_api(&self) -> &dyn crate::apis::SyncTargetApi {
        self.sync_target_api.as_ref()
    }

    pub fn upgrade_api(&self) -> &dyn crate::apis::UpgradeApi {
        self.upgrade_api.as_ref()
    }

    pub fn upgrade_cluster_api(&self) -> &dyn crate::apis::UpgradeClusterApi {
        self.upgrade_cluster_api.as_ref()
    }

    pub fn worm_api(&self) -> &dyn crate::apis::WormApi {
        self.worm_api.as_ref()
    }

    pub fn zones_api(&self) -> &dyn crate::apis::ZonesApi {
        self.zones_api.as_ref()
    }

    pub fn zones_summary_api(&self) -> &dyn crate::apis::ZonesSummaryApi {
        self.zones_summary_api.as_ref()
    }
}
