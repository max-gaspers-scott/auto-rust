#!/usr/bin/env python3
"""
MCP Server for Auto Rust AI Agent
Exposes custom Rust project generation tools to Goose AI agent
"""

import subprocess
import os
import sys
from typing import Annotated
from fastmcp import FastMCP

mcp = FastMCP("auto-rust")

# Path to the compiled binary - will use cargo run if not found
BINARY_PATH = os.path.join(os.path.dirname(__file__), "target", "release", "auto-rust")


def run_auto_rust(command: str, **kwargs) -> str:
    """Execute the auto-rust binary with the given command"""
    try:
        # Build the command
        cmd = []
        
        # Use cargo run if binary doesn't exist
        if not os.path.exists(BINARY_PATH):
            cmd = ["cargo", "run", "--"]
        else:
            cmd = [BINARY_PATH]
        
        cmd.extend(["--what-to-make", command])
        
        # Add optional arguments
        if "sql" in kwargs:
            cmd.extend(["--sql", kwargs["sql"]])
        if "dto_name" in kwargs:
            cmd.extend(["--dto-name", kwargs["dto_name"]])
        if "return_fields" in kwargs:
            cmd.extend(["--retun-fields", kwargs["return_fields"]])
        
        # Run in the directory where the command is executed
        result = subprocess.run(
            cmd,
            cwd=os.getcwd(),
            capture_output=True,
            text=True,
            timeout=120
        )
        
        output = result.stdout + result.stderr
        if result.returncode != 0:
            return f"Command failed with return code {result.returncode}:\n{output}"
        
        return output if output else "Command completed successfully"
    
    except subprocess.TimeoutExpired:
        return "Command timed out after 120 seconds"
    except Exception as e:
        return f"Error executing command: {str(e)}"


@mcp.tool()
def setup_rust_project() -> str:
    """
    Initialize a new Rust web project with Axum, PostgreSQL, Docker, and React frontend.
    Creates backend directory, Dockerfile, docker-compose.yaml, and boilerplate code.
    Run this from the parent directory where you want to create the project.
    """
    return run_auto_rust("setup")


@mcp.tool()
def generate_sql(description: Annotated[str, "Natural language description of the database schema to generate"]) -> str:
    """
    Generate PostgreSQL migration files from natural language description.
    Creates SQL files with CREATE TABLE statements based on the description.
    
    Example: "make a database to track information about users with name, email, and password"
    """
    return run_auto_rust("sql", sql=description)


@mcp.tool()
def add_get_endpoint(
    dto_name: Annotated[str, "Name of the database table/model (e.g., 'users', 'posts')"],
    return_fields: Annotated[str, "Comma-separated list of fields to return (e.g., 'id,name,email')"]
) -> str:
    """
    Generate a GET endpoint for a database model.
    Creates the route handler to fetch records from the database.
    """
    return run_auto_rust("get_endpoint", dto_name=dto_name, return_fields=return_fields)


@mcp.tool()
def add_post_endpoint(dto_name: Annotated[str, "Name of the database table/model to create POST endpoint for"]) -> str:
    """
    Generate a POST endpoint for a database model.
    Creates the route handler to insert new records into the database.
    """
    return run_auto_rust("post", dto_name=dto_name)


@mcp.tool()
def add_minio_integration() -> str:
    """
    Add MinIO object storage integration to the Rust project.
    Adds code for file upload/download with presigned URLs to src/main.rs.
    """
    return run_auto_rust("minio")


@mcp.tool()
def add_python_service() -> str:
    """
    Add a Python FastAPI service to the project.
    Creates a FastAPI template and integrates it with the Rust backend.
    """
    return run_auto_rust("python")


@mcp.tool()
def generate_sql_crate() -> str:
    """
    Generate a SQL crate/module for the project.
    Creates a separate crate for database operations.
    """
    return run_auto_rust("sql_crate")


@mcp.tool()
def make_structs_public() -> str:
    """
    Make all structs in backend/src/models public.
    Adds 'pub' keyword to struct definitions for external access.
    """
    return run_auto_rust("pub_struct")


@mcp.tool()
def show_help() -> str:
    """
    Show available auto-rust commands and their usage.
    Lists all supported operations for scaffolding Rust projects.
    """
    return run_auto_rust("h")


if __name__ == "__main__":
    mcp.run()
