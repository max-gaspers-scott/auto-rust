use indoc::formatdoc;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, ContentBlock, ErrorCode, ErrorData, Implementation, InitializeResult,
        MetaObject, ServerCapabilities, ServerInfo,
    },
    schemars::JsonSchema,
    service::RequestContext,
    tool, tool_handler, tool_router, RoleServer, ServerHandler,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use auto_rust::add_one_sql_funk::{add_one_post, add_one_sql_funk};
use auto_rust::fix_structs::add_pub;
use auto_rust::gen_sql::gen_sql;
use auto_rust::models::handeler_meta_data::HandelerMetaData;
use auto_rust::request_ai::mgs_proxy;
use auto_rust::setup::setup;

const WORKING_DIR_HEADER: &str = "agent-working-dir";

fn extract_working_dir_from_meta(meta: &MetaObject) -> Option<PathBuf> {
    meta.0
        .get(WORKING_DIR_HEADER)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

fn internal_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
}

/// Run a blocking scaffolding closure on the blocking thread pool, first
/// switching the process working directory to `working_dir` when provided.
/// The auto-rust functions operate relative to the current directory, so the
/// working dir supplied by the agent must be applied before invoking them.
async fn run_scaffold<F>(working_dir: Option<PathBuf>, f: F) -> Result<String, ErrorData>
where
    F: FnOnce() -> Result<String, Box<dyn std::error::Error + Send + Sync>> + Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        if let Some(dir) = working_dir {
            std::env::set_current_dir(&dir)?;
        }
        f()
    })
    .await
    .map_err(internal_err)?
    .map_err(internal_err)
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenerateSqlParams {
    /// Natural-language description of the database to design.
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AddGetEndpointParams {
    /// The model/DTO name (matches a file in backend/src/models).
    pub dto_name: String,
    /// Space-separated list of columns to return.
    pub fields_to_return: String,
    /// The column to match on (the SELECT ... WHERE column).
    pub match_column: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AddPostEndpointParams {
    /// The model/DTO name (matches a file in backend/src/models).
    pub dto_name: String,
}

#[derive(Clone)]
pub struct AutoRustServer {
    tool_router: ToolRouter<Self>,
    instructions: String,
}

impl Default for AutoRustServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router(router = tool_router)]
impl AutoRustServer {
    pub fn new() -> Self {
        let instructions = formatdoc! {r#"
            This extension scaffolds a Rust (axum + sqlx + Postgres) backend plus a
            React frontend from natural language. Typical flow:
              1. setup_rust_project   - create the backend/frontend/docker skeleton
              2. generate_sql         - write migrations/0001_data.sql from a description
              3. make_structs_public  - after models are generated from the schema
              4. add_get_endpoint / add_post_endpoint - append handlers to main.rs
            All tools operate inside the current working directory.
            "#};

        Self {
            tool_router: Self::tool_router(),
            instructions,
        }
    }

    #[tool(
        name = "setup_rust_project",
        description = "Scaffold a new Rust axum backend, React frontend, Dockerfile and docker-compose in the current directory."
    )]
    pub async fn setup_rust_project(
        &self,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let working_dir = extract_working_dir_from_meta(&context.meta);
        let msg = run_scaffold(working_dir, || {
            setup()?;
            Ok("Project scaffold created.".to_string())
        })
        .await?;
        Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
    }

    #[tool(
        name = "generate_sql",
        description = "Generate a Postgres schema from a natural-language description and write it to migrations/0001_data.sql."
    )]
    pub async fn generate_sql(
        &self,
        params: Parameters<GenerateSqlParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;
        let working_dir = extract_working_dir_from_meta(&context.meta);
        let msg = run_scaffold(working_dir, move || {
            gen_sql(params.description, mgs_proxy)?;
            Ok("SQL schema written to migrations/0001_data.sql.".to_string())
        })
        .await?;
        Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
    }

    #[tool(
        name = "make_structs_public",
        description = "Make the generated model structs public and derive sqlx/serde traits on them."
    )]
    pub async fn make_structs_public(
        &self,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let working_dir = extract_working_dir_from_meta(&context.meta);
        let msg = run_scaffold(working_dir, || {
            add_pub()?;
            Ok("Model structs updated.".to_string())
        })
        .await?;
        Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
    }

    #[tool(
        name = "add_get_endpoint",
        description = "Append an axum GET handler that selects rows from a table by a matching column."
    )]
    pub async fn add_get_endpoint(
        &self,
        params: Parameters<AddGetEndpointParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;
        let working_dir = extract_working_dir_from_meta(&context.meta);
        let msg = run_scaffold(working_dir, move || {
            add_one_sql_funk(HandelerMetaData {
                name: params.dto_name,
                fields_to_retrun: params.fields_to_return,
                field_to_filtter: Some(params.match_column),
            })?;
            Ok("GET endpoint appended to backend/src/main.rs.".to_string())
        })
        .await?;
        Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
    }

    #[tool(
        name = "add_post_endpoint",
        description = "Append an axum POST handler that inserts a row into a table."
    )]
    pub async fn add_post_endpoint(
        &self,
        params: Parameters<AddPostEndpointParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;
        let working_dir = extract_working_dir_from_meta(&context.meta);
        let msg = run_scaffold(working_dir, move || {
            add_one_post(params.dto_name)?;
            Ok("POST endpoint appended to backend/src/main.rs.".to_string())
        })
        .await?;
        Ok(CallToolResult::success(vec![ContentBlock::text(msg)]))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for AutoRustServer {
    fn get_info(&self) -> ServerInfo {
        InitializeResult::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "goose-autorust",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(self.instructions.clone())
    }
}
