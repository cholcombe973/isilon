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

pub struct JobApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> JobApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> JobApiClient<C> {
        JobApiClient {
            configuration: configuration,
        }
    }
}

pub trait JobApi {
    fn create_job_job(
        &self,
        job_job: crate::models::JobJobCreateParams,
    ) -> Result<crate::models::CreateJobJobResponse, Error>;
    fn create_job_policy(
        &self,
        job_policy: crate::models::JobPolicyCreateParams,
    ) -> Result<crate::models::CreateResponse, Error>;
    fn delete_job_policy(&self, job_policy_id: &str) -> Result<(), Error>;
    fn get_job_events(
        &self,
        begin: i32,
        end: i32,
        job_id: i32,
        resume: &str,
        job_type: &str,
        timeout_ms: i32,
        state: &str,
        limit: i32,
        key: &str,
    ) -> Result<crate::models::JobEvents, Error>;
    fn get_job_job(
        &self,
        job_job_id: &str,
    ) -> Result<crate::models::JobJobs, Error>;
    fn get_job_job_summary(
        &self,
    ) -> Result<crate::models::JobJobSummary, Error>;
    fn get_job_policy(
        &self,
        job_policy_id: &str,
    ) -> Result<crate::models::JobPolicies, Error>;
    fn get_job_recent(
        &self,
        timeout_ms: i32,
        limit: i32,
    ) -> Result<crate::models::JobRecent, Error>;
    fn get_job_reports(
        &self,
        begin: i32,
        end: i32,
        job_id: i32,
        resume: &str,
        job_type: &str,
        timeout_ms: i32,
        limit: i32,
        key: &str,
        verbose: bool,
    ) -> Result<crate::models::JobReports, Error>;
    fn get_job_statistics(
        &self,
        devid: i32,
        job_id: i32,
    ) -> Result<crate::models::JobStatistics, Error>;
    fn get_job_type(
        &self,
        job_type_id: &str,
    ) -> Result<crate::models::JobTypes, Error>;
    fn get_job_types(
        &self,
        sort: &str,
        show_all: bool,
        dir: &str,
    ) -> Result<crate::models::JobTypesExtended, Error>;
    fn list_job_jobs(
        &self,
        sort: &str,
        resume: &str,
        batch: bool,
        state: &str,
        limit: i32,
        dir: &str,
    ) -> Result<crate::models::JobJobsExtended, Error>;
    fn list_job_policies(
        &self,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Result<crate::models::JobPoliciesExtended, Error>;
    fn update_job_job(
        &self,
        job_job: crate::models::JobJob,
        job_job_id: &str,
    ) -> Result<(), Error>;
    fn update_job_policy(
        &self,
        job_policy: crate::models::JobPolicy,
        job_policy_id: &str,
    ) -> Result<(), Error>;
    fn update_job_type(
        &self,
        job_type: crate::models::JobType,
        job_type_id: &str,
    ) -> Result<(), Error>;
}

impl<C: hyper::client::connect::Connect + 'static + std::marker::Sync + std::marker::Send + Clone> JobApi for JobApiClient<C> {
    fn create_job_job(
        &self,
        job_job: crate::models::JobJobCreateParams,
    ) -> Result<crate::models::CreateJobJobResponse, Error> {
        let uri_str = format!("{}/platform/3/job/jobs", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &job_job,
            hyper::Method::POST,
        )
    }

    fn create_job_policy(
        &self,
        job_policy: crate::models::JobPolicyCreateParams,
    ) -> Result<crate::models::CreateResponse, Error> {
        let uri_str = format!("{}/platform/1/job/policies", self.configuration.base_path);
        query(
            self.configuration.borrow(),
            &uri_str,
            &job_policy,
            hyper::Method::POST,
        )
    }

    fn delete_job_policy(&self, job_policy_id: &str) -> Result<(), Error> {
        let uri_str = format!(
            "{}/platform/1/job/policies/{JobPolicyId}",
            self.configuration.base_path,
            JobPolicyId = job_policy_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::DELETE,
        )
    }

    fn get_job_events(
        &self,
        begin: i32,
        end: i32,
        job_id: i32,
        resume: &str,
        job_type: &str,
        timeout_ms: i32,
        state: &str,
        limit: i32,
        key: &str,
    ) -> Result<crate::models::JobEvents, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("begin", &begin.to_string())
            .append_pair("end", &end.to_string())
            .append_pair("job_id", &job_id.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("job_type", &job_type.to_string())
            .append_pair("timeout_ms", &timeout_ms.to_string())
            .append_pair("state", &state.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("key", &key.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/job/events?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_job(
        &self,
        job_job_id: &str,
    ) -> Result<crate::models::JobJobs, Error> {
        let uri_str = format!(
            "{}/platform/3/job/jobs/{JobJobId}",
            self.configuration.base_path,
            JobJobId = job_job_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_job_summary(
        &self,
    ) -> Result<crate::models::JobJobSummary, Error> {
        let uri_str = format!(
            "{}/platform/1/job/job-summary",
            self.configuration.base_path
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_policy(
        &self,
        job_policy_id: &str,
    ) -> Result<crate::models::JobPolicies, Error> {
        let uri_str = format!(
            "{}/platform/1/job/policies/{JobPolicyId}",
            self.configuration.base_path,
            JobPolicyId = job_policy_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_recent(
        &self,
        timeout_ms: i32,
        limit: i32,
    ) -> Result<crate::models::JobRecent, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("timeout_ms", &timeout_ms.to_string())
            .append_pair("limit", &limit.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/job/recent?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_reports(
        &self,
        begin: i32,
        end: i32,
        job_id: i32,
        resume: &str,
        job_type: &str,
        timeout_ms: i32,
        limit: i32,
        key: &str,
        verbose: bool,
    ) -> Result<crate::models::JobReports, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("begin", &begin.to_string())
            .append_pair("end", &end.to_string())
            .append_pair("job_id", &job_id.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("job_type", &job_type.to_string())
            .append_pair("timeout_ms", &timeout_ms.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("key", &key.to_string())
            .append_pair("verbose", &verbose.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/3/job/reports?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_statistics(
        &self,
        devid: i32,
        job_id: i32,
    ) -> Result<crate::models::JobStatistics, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("devid", &devid.to_string())
            .append_pair("job_id", &job_id.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/job/statistics?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_type(
        &self,
        job_type_id: &str,
    ) -> Result<crate::models::JobTypes, Error> {
        let uri_str = format!(
            "{}/platform/1/job/types/{JobTypeId}",
            self.configuration.base_path,
            JobTypeId = job_type_id
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn get_job_types(
        &self,
        sort: &str,
        show_all: bool,
        dir: &str,
    ) -> Result<crate::models::JobTypesExtended, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("show_all", &show_all.to_string())
            .append_pair("dir", &dir.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/job/types?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_job_jobs(
        &self,
        sort: &str,
        resume: &str,
        batch: bool,
        state: &str,
        limit: i32,
        dir: &str,
    ) -> Result<crate::models::JobJobsExtended, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("resume", &resume.to_string())
            .append_pair("batch", &batch.to_string())
            .append_pair("state", &state.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .finish();
        let uri_str = format!("{}/platform/3/job/jobs?{}", self.configuration.base_path, q);
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn list_job_policies(
        &self,
        sort: &str,
        limit: i32,
        dir: &str,
        resume: &str,
    ) -> Result<crate::models::JobPoliciesExtended, Error> {
        let q = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", &sort.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("dir", &dir.to_string())
            .append_pair("resume", &resume.to_string())
            .finish();
        let uri_str = format!(
            "{}/platform/1/job/policies?{}",
            self.configuration.base_path, q
        );
        query(
            self.configuration.borrow(),
            &uri_str,
            &"",
            hyper::Method::GET,
        )
    }

    fn update_job_job(
        &self,
        job_job: crate::models::JobJob,
        job_job_id: &str,
    ) -> Result<(), Error> {
        let uri_str = format!(
            "{}/platform/3/job/jobs/{JobJobId}",
            self.configuration.base_path,
            JobJobId = job_job_id
        );
        put(self.configuration.borrow(), &uri_str, &job_job)
    }

    fn update_job_policy(
        &self,
        job_policy: crate::models::JobPolicy,
        job_policy_id: &str,
    ) -> Result<(), Error> {
        let uri_str = format!(
            "{}/platform/1/job/policies/{JobPolicyId}",
            self.configuration.base_path,
            JobPolicyId = job_policy_id
        );
        put(self.configuration.borrow(), &uri_str, &job_policy)
    }

    fn update_job_type(
        &self,
        job_type: crate::models::JobType,
        job_type_id: &str,
    ) -> Result<(), Error> {
        let uri_str = format!(
            "{}/platform/1/job/types/{JobTypeId}",
            self.configuration.base_path,
            JobTypeId = job_type_id
        );
        put(self.configuration.borrow(), &uri_str, &job_type)
    }
}
