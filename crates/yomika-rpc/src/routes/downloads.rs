//! Downloads registry endpoints.
//!
//! - `POST /downloads` — start a model-package download (non-blocking).
//! - `GET /downloads`  — snapshot of every in-flight + recently-finished
//!   package download. Clients poll while any are in flight.
//!
//! `DELETE /operations/{id}` cancels an active transfer and removes its
//! partial file before publishing a terminal status.

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa_axum::{router::OpenApiRouter, routes};
use yomika_core::DownloadProgress;
use yomika_runtime::packages::PackageCatalog;

use crate::AppState;
use crate::error::{ApiError, ApiResult};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::default()
        .routes(routes!(start_download))
        .routes(routes!(list_downloads))
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListDownloadsResponse {
    pub downloads: Vec<DownloadProgress>,
}

#[utoipa::path(
    get,
    path = "/downloads",
    responses((status = 200, body = ListDownloadsResponse))
)]
async fn list_downloads(State(app): State<AppState>) -> ApiResult<Json<ListDownloadsResponse>> {
    let downloads_state = app.downloads();
    let downloads = downloads_state.iter().map(|e| e.value().clone()).collect();
    Ok(Json(ListDownloadsResponse { downloads }))
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StartDownloadRequest {
    /// Package id declared by `declare_hf_model_package!`, or a local LLM
    /// operation id from `GET /llm/catalog` (for example `"llm:qwen3.5-2b"`).
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StartDownloadResponse {
    /// Operation id. Reusing the package id keeps ids meaningful for clients
    /// watching progress events.
    pub operation_id: String,
}

#[utoipa::path(
    post,
    path = "/downloads",
    request_body = StartDownloadRequest,
    responses((status = 202, body = StartDownloadResponse))
)]
async fn start_download(
    State(app): State<AppState>,
    Json(req): Json<StartDownloadRequest>,
) -> ApiResult<(StatusCode, Json<StartDownloadResponse>)> {
    let operation_id = req.model_id.clone();
    let runtime = app.runtime();
    if runtime.downloads().is_active(&operation_id) {
        return Err(ApiError::conflict(format!(
            "download {operation_id} is already running"
        )));
    }

    if operation_id.starts_with("llm:") {
        let model_id = operation_id
            .strip_prefix("llm:")
            .expect("prefix checked above")
            .to_string();
        let expected_operation_id = yomika_app::llm::local_model_download_id(&model_id)
            .map_err(|_| ApiError::not_found(format!("unknown local model {model_id}")))?;
        if operation_id != expected_operation_id {
            return Err(ApiError::not_found(format!(
                "unknown local model {model_id}"
            )));
        }
        tokio::spawn(async move {
            if let Err(error) = yomika_app::llm::download_local_model(&runtime, &model_id).await {
                tracing::error!(model = model_id, "download failed: {error:#}");
            }
        });
    } else {
        let catalog = PackageCatalog::discover();
        let pkg = catalog
            .all()
            .find(|package| package.id == operation_id)
            .ok_or_else(|| ApiError::not_found(format!("unknown package {operation_id}")))?;
        tokio::spawn(async move {
            if let Err(error) = (pkg.ensure)(&runtime).await {
                tracing::error!(package = pkg.id, "download failed: {error:#}");
            }
        });
    }
    Ok((
        StatusCode::ACCEPTED,
        Json(StartDownloadResponse { operation_id }),
    ))
}
