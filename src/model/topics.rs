//! Topics module.
//!
//! ## Overview
//! This module contains the topics for the DS Event Stream.
//!
//! ## Features
//! * Get the topics for the DS Event Stream.
//!
//! ### Example
//! ```
//! use ds_event_stream_rs_sdk::model::topics::Topic;
//!
//! let topic = Topic::DsPipelineJobRequested;
//! assert_eq!(topic.to_string(), "ds.pipeline..job.requested.v1");
//! ```

use strum::{AsRefStr, Display, EnumString};

// region: --> Topic

///
/// This enum contains all the topics for the DS Event Stream.
///
/// # Topics
///
/// * `IdpIdentityUserCreated` - The event when an identity user is created.
/// * `IdpIdentityUserUpdated` - The event when an identity user is updated.
/// * `IdpIdentityUserDeleted` - The event when an identity user is deleted.
/// * `IdpIdentityUserAuthenticated` - The event when an identity user is authenticated.
/// * `IdpIdentityTenantCreated` - The event when an identity tenant is created.
/// * `IdpIdentityTenantUpdated` - The event when an identity tenant is updated.
/// * `IdpIdentityTenantDeleted` - The event when an identity tenant is deleted.
///
/// * `DsPipelineJobRequested` - The event when a pipeline job is requested.
/// * `DsPipelineJobStarted` - The event when a pipeline job is started.
/// * `DsPipelineJobCompleted` - The event when a pipeline job is completed.
/// * `DsPipelineJobFailed` - The event when a pipeline job is failed.
///
/// * `DsPipelineInjectionTaskRequested` - The event when a pipeline injection task is requested.
/// * `DsPipelineInjectionTaskQueued` - The event when a pipeline injection task is queued.
/// * `DsPipelineInjectionTaskStarted` - The event when a pipeline injection task is started.
/// * `DsPipelineInjectionTaskCompleted` - The event when a pipeline injection task is completed.
/// * `DsPipelineInjectionTaskFailed` - The event when a pipeline injection task is failed.
/// * `DsPipelineInjectionMetricCreated` - The event when a pipeline injection metric is created.
///
/// * `DsPipelineTransformTaskRequested` - The event when a pipeline transform task is requested.
/// * `DsPipelineTransformTaskQueued` - The event when a pipeline transform task is queued.
/// * `DsPipelineTransformTaskStarted` - The event when a pipeline transform task is started.
/// * `DsPipelineTransformTaskCompleted` - The event when a pipeline transform task is completed.
/// * `DsPipelineTransformTaskFailed` - The event when a pipeline transform task is failed.
/// * `DsPipelineTransformMetricCreated` - The event when a pipeline transform metric is created.
///
/// * `DsPipelineMigratorTaskRequested` - The event when a pipeline migrator task is requested.
/// * `DsPipelineMigratorTaskQueued` - The event when a pipeline migrator task is queued.
/// * `DsPipelineMigratorTaskStarted` - The event when a pipeline migrator task is started.
/// * `DsPipelineMigratorTaskCompleted` - The event when a pipeline migrator task is completed.
/// * `DsPipelineMigratorTaskFailed` - The event when a pipeline migrator task is failed.
/// * `DsPipelineMigratorMetricCreated` - The event when a pipeline migrator metric is created.
///
/// * `DsPipelineSynchronizerTaskRequested` - The event when a pipeline synchronizer task is requested.
/// * `DsPipelineSynchronizerTaskQueued` - The event when a pipeline synchronizer task is queued.
/// * `DsPipelineSynchronizerTaskStarted` - The event when a pipeline synchronizer task is started.
/// * `DsPipelineSynchronizerTaskCompleted` - The event when a pipeline synchronizer task is completed.
/// * `DsPipelineSynchronizerTaskFailed` - The event when a pipeline synchronizer task is failed.
/// * `DsPipelineSynchronizerMetricCreated` - The event when a pipeline synchronizer metric is created.
///
/// * `DsPipelineCloneJobRequested` - The event when a pipeline clone job is requested.
/// * `DsPipelineCloneJobQueued` - The event when a pipeline clone job is queued.
/// * `DsPipelineCloneJobStarted` - The event when a pipeline clone job is started.
/// * `DsPipelineCloneJobCompleted` - The event when a pipeline clone job is completed.
/// * `DsPipelineCloneJobFailed` - The event when a pipeline clone job is failed.
/// * `DsPipelineCloneMetricCreated` - The event when a pipeline clone metric is created.
///
/// * `DsWorkflowPipelineJobRequested` - The event when a workflow pipeline job is requested.
/// * `DsWorkflowPipelineJobQueued` - The event when a workflow pipeline job is queued.
/// * `DsWorkflowPipelineJobStarted` - The event when a workflow pipeline job is started.
/// * `DsWorkflowPipelineJobCompleted` - The event when a workflow pipeline job is completed.
/// * `DsWorkflowPipelineJobFailed` - The event when a workflow pipeline job is failed.
/// * `DsWorkflowPipelineTaskStarted` - The event when a workflow pipeline task is started.
/// * `DsWorkflowPipelineTaskCompleted` - The event when a workflow pipeline task is completed.
/// * `DsWorkflowPipelineTaskFailed` - The event when a workflow pipeline task is failed.
/// * `DsWorkflowPipelineCreated` - The event when a workflow pipeline is created.
/// * `DsWorkflowPipelineUpdated` - The event when a workflow pipeline is updated.
/// * `DsWorkflowPipelineDeleted` - The event when a workflow pipeline is deleted.
/// * `DsWorkflowDatasetCreated` - The event when a workflow dataset is created.
/// * `DsWorkflowDatasetUpdated` - The event when a workflow dataset is updated.
/// * `DsWorkflowDatasetDeleted` - The event when a workflow dataset is deleted.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, AsRefStr, EnumString)]
pub enum Topic {
    // IDP Identity User Events
    #[strum(serialize = "idp.identity..user.created.v1")]
    IdpIdentityUserCreated,
    #[strum(serialize = "idp.identity..user.updated.v1")]
    IdpIdentityUserUpdated,
    #[strum(serialize = "idp.identity..user.deleted.v1")]
    IdpIdentityUserDeleted,
    #[strum(serialize = "idp.identity..user.authenticated.v1")]
    IdpIdentityUserAuthenticated,

    // IDP Identity Tenant Events
    #[strum(serialize = "idp.identity..tenant.created.v1")]
    IdpIdentityTenantCreated,
    #[strum(serialize = "idp.identity..tenant.updated.v1")]
    IdpIdentityTenantUpdated,
    #[strum(serialize = "idp.identity..tenant.deleted.v1")]
    IdpIdentityTenantDeleted,

    // DS Pipeline Job Events
    #[strum(serialize = "ds.pipeline..job.requested.v1")]
    DsPipelineJobRequested,
    #[strum(serialize = "ds.pipeline..job.started.v1")]
    DsPipelineJobStarted,
    #[strum(serialize = "ds.pipeline..job.completed.v1")]
    DsPipelineJobCompleted,
    #[strum(serialize = "ds.pipeline..job.failed.v1")]
    DsPipelineJobFailed,

    // DS Pipeline Injection Task Events
    #[strum(serialize = "ds.pipeline.injection.task.requested.v1")]
    DsPipelineInjectionTaskRequested,
    #[strum(serialize = "ds.pipeline.injection.task.queued.v1")]
    DsPipelineInjectionTaskQueued,
    #[strum(serialize = "ds.pipeline.injection.task.started.v1")]
    DsPipelineInjectionTaskStarted,
    #[strum(serialize = "ds.pipeline.injection.task.completed.v1")]
    DsPipelineInjectionTaskCompleted,
    #[strum(serialize = "ds.pipeline.injection.task.failed.v1")]
    DsPipelineInjectionTaskFailed,
    #[strum(serialize = "ds.pipeline.injection.metric.created.v1")]
    DsPipelineInjectionMetricCreated,

    // DS Pipeline Transform Task Events
    #[strum(serialize = "ds.pipeline.transform.task.requested.v1")]
    DsPipelineTransformTaskRequested,
    #[strum(serialize = "ds.pipeline.transform.task.queued.v1")]
    DsPipelineTransformTaskQueued,
    #[strum(serialize = "ds.pipeline.transform.task.started.v1")]
    DsPipelineTransformTaskStarted,
    #[strum(serialize = "ds.pipeline.transform.task.completed.v1")]
    DsPipelineTransformTaskCompleted,
    #[strum(serialize = "ds.pipeline.transform.task.failed.v1")]
    DsPipelineTransformTaskFailed,
    #[strum(serialize = "ds.pipeline.transform.metric.created.v1")]
    DsPipelineTransformMetricCreated,

    // DS Pipeline Migrator Task Events
    #[strum(serialize = "ds.pipeline.migrator.task.requested.v1")]
    DsPipelineMigratorTaskRequested,
    #[strum(serialize = "ds.pipeline.migrator.task.queued.v1")]
    DsPipelineMigratorTaskQueued,
    #[strum(serialize = "ds.pipeline.migrator.task.started.v1")]
    DsPipelineMigratorTaskStarted,
    #[strum(serialize = "ds.pipeline.migrator.task.completed.v1")]
    DsPipelineMigratorTaskCompleted,
    #[strum(serialize = "ds.pipeline.migrator.task.failed.v1")]
    DsPipelineMigratorTaskFailed,
    #[strum(serialize = "ds.pipeline.migrator.metric.created.v1")]
    DsPipelineMigratorMetricCreated,

    // DS Pipeline Synchronizer Task Events
    #[strum(serialize = "ds.pipeline.synchronizer.task.requested.v1")]
    DsPipelineSynchronizerTaskRequested,
    #[strum(serialize = "ds.pipeline.synchronizer.task.queued.v1")]
    DsPipelineSynchronizerTaskQueued,
    #[strum(serialize = "ds.pipeline.synchronizer.task.started.v1")]
    DsPipelineSynchronizerTaskStarted,
    #[strum(serialize = "ds.pipeline.synchronizer.task.completed.v1")]
    DsPipelineSynchronizerTaskCompleted,
    #[strum(serialize = "ds.pipeline.synchronizer.task.failed.v1")]
    DsPipelineSynchronizerTaskFailed,
    #[strum(serialize = "ds.pipeline.synchronizer.metric.created.v1")]
    DsPipelineSynchronizerMetricCreated,

    // DS Pipeline Clone Job Events
    #[strum(serialize = "ds.pipeline.clone.job.requested.v1")]
    DsPipelineCloneJobRequested,
    #[strum(serialize = "ds.pipeline.clone.job.queued.v1")]
    DsPipelineCloneJobQueued,
    #[strum(serialize = "ds.pipeline.clone.job.started.v1")]
    DsPipelineCloneJobStarted,
    #[strum(serialize = "ds.pipeline.clone.job.completed.v1")]
    DsPipelineCloneJobCompleted,
    #[strum(serialize = "ds.pipeline.clone.job.failed.v1")]
    DsPipelineCloneJobFailed,
    #[strum(serialize = "ds.pipeline.clone.metric.created.v1")]
    DsPipelineCloneMetricCreated,

    // DS Workflow Pipeline Job Events
    #[strum(serialize = "ds.workflow.pipeline.job.requested.v1")]
    DsWorkflowPipelineJobRequested,
    #[strum(serialize = "ds.workflow.pipeline.job.queued.v1")]
    DsWorkflowPipelineJobQueued,
    #[strum(serialize = "ds.workflow.pipeline.job.started.v1")]
    DsWorkflowPipelineJobStarted,
    #[strum(serialize = "ds.workflow.pipeline.job.completed.v1")]
    DsWorkflowPipelineJobCompleted,
    #[strum(serialize = "ds.workflow.pipeline.job.failed.v1")]
    DsWorkflowPipelineJobFailed,

    // DS Workflow Pipeline Task Events
    #[strum(serialize = "ds.workflow.pipeline.task.started.v1")]
    DsWorkflowPipelineTaskStarted,
    #[strum(serialize = "ds.workflow.pipeline.task.completed.v1")]
    DsWorkflowPipelineTaskCompleted,
    #[strum(serialize = "ds.workflow.pipeline.task.failed.v1")]
    DsWorkflowPipelineTaskFailed,

    // DS Workflow Pipeline Events
    #[strum(serialize = "ds.workflow..pipeline.created.v1")]
    DsWorkflowPipelineCreated,
    #[strum(serialize = "ds.workflow..pipeline.updated.v1")]
    DsWorkflowPipelineUpdated,
    #[strum(serialize = "ds.workflow..pipeline.deleted.v1")]
    DsWorkflowPipelineDeleted,

    // DS Workflow Dataset Events
    #[strum(serialize = "ds.workflow..dataset.created.v1")]
    DsWorkflowDatasetCreated,
    #[strum(serialize = "ds.workflow..dataset.updated.v1")]
    DsWorkflowDatasetUpdated,
    #[strum(serialize = "ds.workflow..dataset.deleted.v1")]
    DsWorkflowDatasetDeleted,

    // DS Workflow Linked Service Events
    #[strum(serialize = "ds.workflow..linked-service.created.v1")]
    DsWorkflowLinkedServiceCreated,
    #[strum(serialize = "ds.workflow..linked-service.updated.v1")]
    DsWorkflowLinkedServiceUpdated,
    #[strum(serialize = "ds.workflow..linked-service.deleted.v1")]
    DsWorkflowLinkedServiceDeleted,

    // DS Core Provision Job Events
    #[strum(serialize = "ds.core.provision.job.requested.v1")]
    DsCoreProvisionJobRequested,
    #[strum(serialize = "ds.core.provision.job.completed.v1")]
    DsCoreProvisionJobCompleted,
    #[strum(serialize = "ds.core.provision.job.failed.v1")]
    DsCoreProvisionJobFailed,

    // DS Core Config Events
    #[strum(serialize = "ds.core.config.info.updated.v1")]
    DsCoreConfigInfoUpdated,
    #[strum(serialize = "ds.core.config.status.updated.v1")]
    DsCoreConfigStatusUpdated,

    // DS Core Billing Events
    #[strum(serialize = "ds.core.billing.usage.created.v1")]
    DsCoreBillingUsageCreated,

    // All Topics
    #[strum(serialize = "*")]
    AllTopics,
}

// endregion: --> Topic
