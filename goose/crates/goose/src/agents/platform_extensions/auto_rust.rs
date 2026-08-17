use crate::agents::extension::PlatformExtensionContext;
use crate::agents::mcp_client::{Error, McpClientTrait};
use crate::agents::tool_execution::ToolCallContext;
use anyhow::Result;
use async_trait::async_trait;
use indoc::indoc;
use rmcp::model::{
    CallToolResult, ContentBlock, Implementation, InitializeResult, JsonObject, ListToolsResult,
    ServerCapabilities, Tool, ToolAnnotations,
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

pub static EXTENSION_NAME: &str = "auto_rust";

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct SetupProjectParams {}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct GenerateSqlParams {
    /// Description of the SQL database to generate
    description: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct GenerateDockerParams {
    /// Name of the binary to use in the Dockerfile
    binary_name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct AddMinioParams {}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct AddGetEndpointParams {
    /// Name of the DTO/model to create GET endpoint for
    dto_name: String,
    /// Space-separated list of fields to return
    return_fields: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct AddPostEndpointParams {
    /// Name of the DTO/model to create POST endpoint for
    dto_name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct AddPubStructParams {}

pub struct AutoRustClient {
    info: InitializeResult,
    _context: PlatformExtensionContext,
}

impl AutoRustClient {
    pub fn new(context: PlatformExtensionContext) -> Result<Self> {
        let info = InitializeResult::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(
                Implementation::new(EXTENSION_NAME.to_string(), "1.0.0".to_string())
                    .with_title("Auto Rust"),
            )
            .with_instructions(
                indoc! {r#"
                Use the auto_rust extension to scaffold Rust web projects with Axum, PostgreSQL, and Docker.

                Available commands:
                - setup_project: Initialize a new Rust backend with boilerplate
                - generate_sql: Generate PostgreSQL migrations from natural language descriptions
                - generate_docker: Create a multi-stage Dockerfile for the project
                - add_minio: Add MinIO object storage integration
                - add_get_endpoint: Generate a GET endpoint for a database model
                - add_post_endpoint: Generate a POST endpoint for a database model
                - add_pub_struct: Make all structs in backend/src/models public

                Typical workflow:
                1. setup_project - creates backend directory with Cargo project
                2. generate_sql - creates database schema
                3. add_get_endpoint/add_post_endpoint - creates API endpoints
                4. generate_docker - containerize the application
            "#}
                .to_string(),
            );

        Ok(Self {
            info,
            _context: context,
        })
    }

    fn schema<T: JsonSchema>() -> JsonObject {
        serde_json::to_value(schema_for!(T))
            .expect("schema serialization should succeed")
            .as_object()
            .expect("schema should serialize to an object")
            .clone()
    }

    async fn handle_setup_project(
        &self,
        _arguments: Option<JsonObject>,
    ) -> Result<Vec<ContentBlock>, String> {
        // Call the setup function from auto-rust
        match auto_rust::setup::setup() {
            Ok(_) => Ok(vec![ContentBlock::text(
                "Successfully set up Rust project with backend, Dockerfile, and docker-compose.yaml",
            )]),
            Err(e) => Err(format!("Failed to setup project: {}", e)),
        }
    }

    async fn handle_generate_sql(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<Vec<ContentBlock>, String> {
        let description = arguments
            .as_ref()
            .ok_or("Missing arguments")?
            .get("description")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: description")?
            .to_string();

        match auto_rust::gen_sql::gen_sql(description, false).await {
            Ok(content) => Ok(vec![ContentBlock::text(format!(
                "Successfully generated SQL ({} bytes)",
                content.len()
            ))]),
            Err(e) => Err(format!("Failed to generate SQL: {}", e)),
        }
    }

    async fn handle_generate_docker(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<Vec<ContentBlock>, String> {
        let binary_name = arguments
            .as_ref()
            .ok_or("Missing arguments")?
            .get("binary_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: binary_name")?;

        let dockerfile_path = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join("Dockerfile");

        match auto_rust::gen_docker::gen_docker(&dockerfile_path, binary_name) {
            Ok(_) => Ok(vec![ContentBlock::text(format!(
                "Successfully generated Dockerfile at {}",
                dockerfile_path.display()
            ))]),
            Err(e) => Err(format!("Failed to generate Dockerfile: {}", e)),
        }
    }

    async fn handle_add_minio(
        &self,
        _arguments: Option<JsonObject>,
    ) -> Result<Vec<ContentBlock>, String> {
        let main_path = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join("src/main.rs");

        match auto_rust::add_minio::add_minio(&main_path) {
            Ok(_) => Ok(vec![ContentBlock::text(
                "Successfully added MinIO integration to src/main.rs",
            )]),
            Err(e) => Err(format!("Failed to add MinIO: {}", e)),
        }
    }

    async fn handle_add_get_endpoint(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<Vec<ContentBlock>, String> {
        let args_obj = arguments.as_ref().ok_or("Missing arguments")?;

        let dto_name = args_obj
            .get("dto_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: dto_name")?
            .to_string();

        let return_fields = args_obj
            .get("return_fields")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: return_fields")?
            .to_string();

        match auto_rust::add_one_sql_funk::add_one_sql_funk(dto_name.clone(), return_fields) {
            Ok(_) => Ok(vec![ContentBlock::text(format!(
                "Successfully added GET endpoint for {}",
                dto_name
            ))]),
            Err(e) => Err(format!("Failed to add GET endpoint: {}", e)),
        }
    }

    async fn handle_add_post_endpoint(
        &self,
        arguments: Option<JsonObject>,
    ) -> Result<Vec<ContentBlock>, String> {
        let dto_name = arguments
            .as_ref()
            .ok_or("Missing arguments")?
            .get("dto_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: dto_name")?
            .to_string();

        match auto_rust::add_one_sql_funk::add_one_post(dto_name.clone()) {
            Ok(_) => Ok(vec![ContentBlock::text(format!(
                "Successfully added POST endpoint for {}",
                dto_name
            ))]),
            Err(e) => Err(format!("Failed to add POST endpoint: {}", e)),
        }
    }

    async fn handle_add_pub_struct(
        &self,
        _arguments: Option<JsonObject>,
    ) -> Result<Vec<ContentBlock>, String> {
        let models_path = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join("backend/src/models");

        match auto_rust::fix_structs::add_pub(&models_path) {
            Ok(_) => Ok(vec![ContentBlock::text(
                "Successfully made all structs in backend/src/models public",
            )]),
            Err(e) => Err(format!("Failed to add pub to structs: {}", e)),
        }
    }

    fn get_tools() -> Vec<Tool> {
        vec![
            Tool::new(
                "setup_project".to_string(),
                "Initialize a new Rust web project with Axum, PostgreSQL, Docker, and React frontend. Creates backend directory, Dockerfile, docker-compose.yaml, and boilerplate code.".to_string(),
                Self::schema::<SetupProjectParams>(),
            )
            .annotate(ToolAnnotations::from_raw(
                Some("Setup Project".to_string()),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
            )),
            Tool::new(
                "generate_sql".to_string(),
                "Generate PostgreSQL migration files from natural language description. Creates migrations/0001_data.sql with CREATE TABLE statements. Uses Gemini API to convert description to SQL.".to_string(),
                Self::schema::<GenerateSqlParams>(),
            )
            .annotate(ToolAnnotations::from_raw(
                Some("Generate SQL".to_string()),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
            )),
            Tool::new(
                "generate_docker".to_string(),
                "Create a multi-stage Dockerfile for the Rust project. Includes builder stage with Rust nightly, frontend build stage with Node, and minimal runtime stage.".to_string(),
                Self::schema::<GenerateDockerParams>(),
            )
            .annotate(ToolAnnotations::from_raw(
                Some("Generate Docker".to_string()),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
            )),
            Tool::new(
                "add_minio".to_string(),
                "Add MinIO object storage integration to the project. Adds necessary imports, handler functions, and dependencies to src/main.rs.".to_string(),
                Self::schema::<AddMinioParams>(),
            )
            .annotate(ToolAnnotations::from_raw(
                Some("Add MinIO".to_string()),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
            )),
            Tool::new(
                "add_get_endpoint".to_string(),
                "Generate a GET endpoint for a database model. Creates query struct and handler function that selects from database by a specified column.".to_string(),
                Self::schema::<AddGetEndpointParams>(),
            )
            .annotate(ToolAnnotations::from_raw(
                Some("Add GET Endpoint".to_string()),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
            )),
            Tool::new(
                "add_post_endpoint".to_string(),
                "Generate a POST endpoint for a database model. Creates handler function that inserts new records into the database table.".to_string(),
                Self::schema::<AddPostEndpointParams>(),
            )
            .annotate(ToolAnnotations::from_raw(
                Some("Add POST Endpoint".to_string()),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
            )),
            Tool::new(
                "add_pub_struct".to_string(),
                "Make all structs in backend/src/models public by adding 'pub' keyword. Useful for fixing visibility issues.".to_string(),
                Self::schema::<AddPubStructParams>(),
            )
            .annotate(ToolAnnotations::from_raw(
                Some("Add Pub Struct".to_string()),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
            )),
        ]
    }
}

#[async_trait]
impl McpClientTrait for AutoRustClient {
    async fn list_tools(
        &self,
        _session_id: &str,
        _next_cursor: Option<String>,
        _cancellation_token: CancellationToken,
    ) -> Result<ListToolsResult, Error> {
        Ok(ListToolsResult {
            tools: Self::get_tools(),
            next_cursor: None,
            meta: None,
            ..Default::default()
        })
    }

    async fn call_tool(
        &self,
        _ctx: &ToolCallContext,
        name: &str,
        arguments: Option<JsonObject>,
        _cancellation_token: CancellationToken,
    ) -> Result<CallToolResult, Error> {
        let content = match name {
            "setup_project" => self.handle_setup_project(arguments).await,
            "generate_sql" => self.handle_generate_sql(arguments).await,
            "generate_docker" => self.handle_generate_docker(arguments).await,
            "add_minio" => self.handle_add_minio(arguments).await,
            "add_get_endpoint" => self.handle_add_get_endpoint(arguments).await,
            "add_post_endpoint" => self.handle_add_post_endpoint(arguments).await,
            "add_pub_struct" => self.handle_add_pub_struct(arguments).await,
            _ => Err(format!("Unknown tool: {}", name)),
        };

        match content {
            Ok(content) => Ok(CallToolResult::success(content)),
            Err(error) => Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                "Error: {}",
                error
            ))])),
        }
    }

    fn get_info(&self) -> Option<&InitializeResult> {
        Some(&self.info)
    }
}
