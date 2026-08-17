# Auto-Rust Goose Integration Setup

This project provides custom tools for Goose AI agent to scaffold Rust web projects.

## What This Does

The MCP (Model Context Protocol) server in `auto_rust_mcp.py` exposes your Rust project generation tools to Goose, allowing the AI agent to:

- Initialize new Rust projects with Axum, PostgreSQL, Docker, and React
- Generate SQL schemas from natural language descriptions  
- Create REST API endpoints (GET/POST) for database models
- Add MinIO object storage integration
- Add Python FastAPI microservices
- Make structs public for external access

## Prerequisites

1. **Python 3.8+** with `fastmcp` package:
   ```bash
   pip install fastmcp
   ```

2. **Rust toolchain** (for cargo commands):
   ```bash
   # Should already be installed if you're developing this project
   cargo --version
   ```

3. **Goose AI agent** installed:
   ```bash
   # Follow instructions at: https://github.com/block/goose
   ```

## Setup

### Option 1: Using the Recipe (Recommended)

1. Make the MCP server executable:
   ```bash
   chmod +x auto_rust_mcp.py
   ```

2. Start Goose with the recipe from another project directory:
   ```bash
   cd /path/to/your/test-project
   goose run --recipe /home/mgs/auto-rust-ai-agent/auto-rust-recipe.yaml -s
   ```
   The `-s` (`--interactive`) flag keeps you in an interactive chat after the
   recipe loads. Without it, Goose runs headless and uses the recipe's `prompt`.

3. Test it by asking Goose:
   ```
   "Can you show me what auto-rust tools are available?"
   "Set up a new Rust project here"
   "Generate SQL for a users table with email and password"
   ```

### Option 2: Manual Extension Configuration

1. Add the extension to your Goose config:
   ```bash
   goose configure extension add
   ```
   
   Then configure:
   - Type: `stdio`
   - Name: `auto-rust`
   - Command: `python3`
   - Args: `/home/mgs/auto-rust-ai-agent/auto_rust_mcp.py`

2. Enable the extension:
   ```bash
   goose configure extension enable auto-rust
   ```

## Testing

### Test the MCP Server Directly

First, verify the MCP server works standalone:

```bash
cd /home/mgs/auto-rust-ai-agent
venv/bin/python auto_rust_mcp.py
```

It should start and display the FastMCP banner, waiting for MCP protocol messages. Press Ctrl+C to exit.

### Test with Goose

1. Create a test project directory:
   ```bash
   mkdir -p ~/test-rust-project
   cd ~/test-rust-project
   ```

2. Start Goose with the auto-rust recipe (interactive):
   ```bash
   goose run --recipe /home/mgs/auto-rust-ai-agent/auto-rust-recipe.yaml -s
   ```

3. Try these commands:
   - "Show me the available auto-rust tools"
   - "Use setup_rust_project to initialize a new project here"
   - "Generate SQL schema for tracking blog posts with title, content, and author"
   - "Add a GET endpoint for the posts table returning id, title, and author"

### Verify Tools are Loaded

In a Goose session, you can check if tools loaded:
```
"What tools do you have access to?"
"List all available extensions"
```

You should see tools like:
- `setup_rust_project`
- `generate_sql`
- `add_get_endpoint`
- `add_post_endpoint`
- `add_minio_integration`
- etc.

## File Structure

```
auto-rust-ai-agent/
├── auto_rust_mcp.py          # MCP server exposing Rust tools to Goose
├── auto-rust-recipe.yaml     # Goose recipe configuration
├── src/                      # Rust source code with tool implementations
│   ├── gen_toml.rs          # Generate Cargo.toml dependencies
│   ├── gen_sql.rs           # Generate SQL from natural language
│   ├── add_minio.rs         # Add MinIO integration
│   └── ...                  # Other tool modules
└── Cargo.toml               # Rust project configuration
```

## Troubleshooting

### "Command not found" errors
- Make sure you're running from a directory where cargo is available
- The MCP server will try `cargo run` if the compiled binary doesn't exist

### Extension not loading
- Check Goose logs: `~/.config/goose/logs/`
- Verify Python can find `fastmcp`: `python3 -c "import fastmcp"`
- Make sure the path to `auto_rust_mcp.py` is absolute in the recipe

### Tools timeout
- The default timeout is 120 seconds. Some operations (like SQL generation with AI) may take time
- Increase `timeout` in the recipe if needed

## Known Issues

⚠️ **Note**: I haven't modified the actual tool implementations in `src/`. If there are issues with the tools themselves (e.g., compilation errors, runtime bugs), those would need to be fixed separately in the Rust code.

The MCP integration will expose whatever the tools currently do - both their features and any existing bugs.
