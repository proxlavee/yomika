//! Per-domain route modules. Each exposes `router()` returning an
//! `OpenApiRouter<ApiState>` that can be merged into the top-level router in
//! `api.rs`.

/// Raw binary content in the HTTP API.
#[derive(utoipa::ToSchema)]
#[schema(value_type = String, format = Binary)]
#[allow(dead_code)]
pub(super) struct BinaryPayload(Vec<u8>);

pub mod ai;
pub mod config;
pub mod downloads;
pub mod fonts;
pub mod history;
pub mod llm;
pub mod meta;
pub mod operations;
pub mod pages;
pub mod pipelines;
pub mod projects;
pub mod storage;
