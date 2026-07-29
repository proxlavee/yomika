//! Model-library location, inventory, and guarded cleanup endpoints.

use std::path::PathBuf;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use utoipa_axum::{router::OpenApiRouter, routes};
use yomika_core::{JobStatus, LlmStateStatus, LlmTargetKind};

use crate::AppState;
use crate::error::{ApiError, ApiResult};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::default()
        .routes(routes!(get_storage))
        .routes(routes!(set_model_location))
        .routes(routes!(clear_models))
        .routes(routes!(delete_local_model))
        .routes(routes!(redownload_local_model))
        .routes(routes!(clear_temporary_cache))
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StorageSummary {
    pub data_path: String,
    pub models_path: String,
    pub custom_models_path: bool,
    pub models_bytes: u64,
    pub temporary_bytes: u64,
    pub downloaded_local_models: usize,
}

#[utoipa::path(get, path = "/storage", responses((status = 200, body = StorageSummary)))]
async fn get_storage(State(app): State<AppState>) -> ApiResult<Json<StorageSummary>> {
    let runtime = app.runtime();
    let runtime_for_scan = runtime.clone();
    let usage =
        tokio::task::spawn_blocking(move || yomika_app::storage::usage(runtime_for_scan.as_ref()))
            .await
            .map_err(join_error)?
            .map_err(ApiError::internal)?;
    let config = app.config.load();
    Ok(Json(StorageSummary {
        data_path: runtime.root().to_string_lossy().into_owned(),
        models_path: runtime.models_root().to_string_lossy().into_owned(),
        custom_models_path: config.data.models_path.is_some(),
        models_bytes: usage.models_bytes,
        temporary_bytes: usage.temporary_bytes,
        downloaded_local_models: usage.downloaded_local_models,
    }))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelLocationMode {
    UseExisting,
    MoveExisting,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetModelLocationRequest {
    pub path: String,
    pub mode: ModelLocationMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetModelLocationResponse {
    pub models_path: String,
    pub copied_bytes: u64,
    pub source_removed: bool,
    pub restart_required: bool,
}

#[utoipa::path(
    put,
    path = "/storage/models/location",
    request_body = SetModelLocationRequest,
    responses((status = 200, body = SetModelLocationResponse))
)]
async fn set_model_location(
    State(app): State<AppState>,
    Json(request): Json<SetModelLocationRequest>,
) -> ApiResult<Json<SetModelLocationResponse>> {
    ensure_storage_idle(&app)?;
    ensure_local_model_idle(&app, None).await?;
    let path = request.path.trim();
    if path.is_empty() {
        return Err(ApiError::bad_request("model-library path cannot be empty"));
    }

    let destination = PathBuf::from(path);
    let move_existing = matches!(request.mode, ModelLocationMode::MoveExisting);
    let runtime = app.runtime();
    let runtime_for_migration = runtime.clone();
    let prepared = tokio::task::spawn_blocking(move || {
        yomika_app::storage::prepare_model_location(
            runtime_for_migration.as_ref(),
            &destination,
            move_existing,
        )
    })
    .await
    .map_err(join_error)?
    .map_err(storage_request_error)?;

    let destination_utf8 =
        Utf8PathBuf::from_path_buf(prepared.destination.clone()).map_err(|path| {
            ApiError::bad_request(format!("path is not valid UTF-8: {}", path.display()))
        })?;
    let mut next = (**app.config.load()).clone();
    let default_models_path = next.data.path.join("models");
    next.data.models_path = if std::fs::canonicalize(default_models_path.as_std_path())
        .is_ok_and(|default| default == prepared.destination)
    {
        None
    } else {
        Some(destination_utf8.clone())
    };
    app.save_config(&next).map_err(ApiError::internal)?;
    app.config.store(Arc::new(next));

    let copied_bytes = prepared.copied_bytes;
    let source_removed = match tokio::task::spawn_blocking(move || {
        yomika_app::storage::finish_model_location(&prepared)
    })
    .await
    {
        Ok(Ok(removed)) => removed,
        Ok(Err(error)) => {
            tracing::warn!("new model location saved, but old cache cleanup failed: {error:#}");
            false
        }
        Err(error) => {
            tracing::warn!("new model location saved, but cleanup task failed: {error}");
            false
        }
    };

    Ok(Json(SetModelLocationResponse {
        models_path: destination_utf8.into_string(),
        copied_bytes,
        source_removed,
        restart_required: true,
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StorageCleanupResponse {
    pub removed_bytes: u64,
}

#[utoipa::path(
    delete,
    path = "/storage/models",
    responses((status = 200, body = StorageCleanupResponse))
)]
async fn clear_models(State(app): State<AppState>) -> ApiResult<Json<StorageCleanupResponse>> {
    ensure_storage_idle(&app)?;
    ensure_local_model_idle(&app, None).await?;
    let runtime = app.runtime();
    let removed_bytes =
        tokio::task::spawn_blocking(move || yomika_app::storage::clear_models(runtime.as_ref()))
            .await
            .map_err(join_error)?
            .map_err(ApiError::internal)?;
    Ok(Json(StorageCleanupResponse { removed_bytes }))
}

#[utoipa::path(
    delete,
    path = "/storage/models/{model_id}",
    params(("model_id" = String, Path, description = "Local model id")),
    responses((status = 200, body = StorageCleanupResponse))
)]
async fn delete_local_model(
    State(app): State<AppState>,
    Path(model_id): Path<String>,
) -> ApiResult<Json<StorageCleanupResponse>> {
    validate_local_model(&model_id)?;
    ensure_storage_idle(&app)?;
    ensure_local_model_idle(&app, Some(&model_id)).await?;
    let runtime = app.runtime();
    let removed_bytes = tokio::task::spawn_blocking(move || {
        yomika_app::storage::remove_local_model(runtime.as_ref(), &model_id)
    })
    .await
    .map_err(join_error)?
    .map_err(ApiError::internal)?;
    Ok(Json(StorageCleanupResponse { removed_bytes }))
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RedownloadModelResponse {
    pub operation_id: String,
    pub removed_bytes: u64,
}

#[utoipa::path(
    post,
    path = "/storage/models/{model_id}/redownload",
    params(("model_id" = String, Path, description = "Local model id")),
    responses((status = 202, body = RedownloadModelResponse))
)]
async fn redownload_local_model(
    State(app): State<AppState>,
    Path(model_id): Path<String>,
) -> ApiResult<(StatusCode, Json<RedownloadModelResponse>)> {
    let operation_id = validate_local_model(&model_id)?;
    ensure_storage_idle(&app)?;
    ensure_local_model_idle(&app, Some(&model_id)).await?;
    let runtime = app.runtime();
    let runtime_for_delete = runtime.clone();
    let model_for_delete = model_id.clone();
    let removed_bytes = tokio::task::spawn_blocking(move || {
        yomika_app::storage::remove_local_model(runtime_for_delete.as_ref(), &model_for_delete)
    })
    .await
    .map_err(join_error)?
    .map_err(ApiError::internal)?;
    tokio::spawn(async move {
        if let Err(error) = yomika_app::llm::download_local_model(&runtime, &model_id).await {
            tracing::error!(model = model_id, "redownload failed: {error:#}");
        }
    });
    Ok((
        StatusCode::ACCEPTED,
        Json(RedownloadModelResponse {
            operation_id,
            removed_bytes,
        }),
    ))
}

#[utoipa::path(
    delete,
    path = "/storage/cache",
    responses((status = 200, body = StorageCleanupResponse))
)]
async fn clear_temporary_cache(
    State(app): State<AppState>,
) -> ApiResult<Json<StorageCleanupResponse>> {
    if app.runtime().downloads().has_active() {
        return Err(ApiError::conflict(
            "cancel active downloads before clearing temporary files",
        ));
    }
    let runtime = app.runtime();
    let removed_bytes = tokio::task::spawn_blocking(move || {
        yomika_app::storage::clear_temporary_cache(runtime.as_ref())
    })
    .await
    .map_err(join_error)?
    .map_err(ApiError::internal)?;
    Ok(Json(StorageCleanupResponse { removed_bytes }))
}

fn validate_local_model(model_id: &str) -> ApiResult<String> {
    yomika_app::llm::local_model_download_id(model_id)
        .map_err(|_| ApiError::not_found(format!("unknown local model {model_id}")))
}

fn ensure_storage_idle(app: &AppState) -> ApiResult<()> {
    if app.runtime().downloads().has_active() {
        return Err(ApiError::conflict(
            "cancel active downloads before changing model storage",
        ));
    }
    if app
        .jobs()
        .iter()
        .any(|job| job.value().status == JobStatus::Running)
    {
        return Err(ApiError::conflict(
            "wait for running jobs to finish before changing model storage",
        ));
    }
    Ok(())
}

async fn ensure_local_model_idle(app: &AppState, model_id: Option<&str>) -> ApiResult<()> {
    let state = app.llm.snapshot().await;
    if !matches!(
        state.status,
        LlmStateStatus::Loading | LlmStateStatus::Ready
    ) {
        return Ok(());
    }
    let Some(target) = state.target else {
        return Ok(());
    };
    if target.kind == LlmTargetKind::Local
        && model_id.is_none_or(|model_id| target.model_id == model_id)
    {
        return Err(ApiError::conflict(
            "unload the local model before changing its files",
        ));
    }
    Ok(())
}

fn storage_request_error(error: anyhow::Error) -> ApiError {
    ApiError::bad_request(format!("{error:#}"))
}

fn join_error(error: tokio::task::JoinError) -> ApiError {
    ApiError::internal(anyhow::Error::new(error))
}
