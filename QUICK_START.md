# Quick Start - Goose + Auto Rust

## TL;DR - Just Let Me Test It!

```bash
# 1. Run the test script
cd /home/mgs/auto-rust-ai-agent
./test_in_project.sh

# 2. In the test directory it creates, run (interactive):
goose run --recipe /home/mgs/auto-rust-ai-agent/auto-rust-recipe.yaml -s

# 3. Ask Goose:
"What auto-rust tools do you have?"
"Use setup_rust_project to create a new project"
```

## Available Tools

When you start Goose with the recipe, it can:

| Tool | What It Does | Example |
|------|-------------|---------|
| `setup_rust_project` | Create full project scaffold | "Set up a new Rust project" |
| `generate_sql` | Generate SQL from description | "Generate SQL for a users table" |
| `add_get_endpoint` | Add GET API endpoint | "Add a GET endpoint for posts" |
| `add_post_endpoint` | Add POST API endpoint | "Add a POST endpoint for users" |
| `add_minio_integration` | Add file storage | "Add MinIO integration" |
| `add_python_service` | Add FastAPI service | "Add a Python service" |
| `generate_sql_crate` | Create SQL crate | "Generate a SQL crate" |
| `make_structs_public` | Make structs public | "Make all model structs public" |

## Testing From Another Project

```bash
# Navigate to your project
cd /path/to/my-new-project

# Start Goose with auto-rust tools (interactive)
goose run --recipe /home/mgs/auto-rust-ai-agent/auto-rust-recipe.yaml -s

# Now Goose can help you scaffold Rust code!
```

## Example Conversations

### Create a Blog Backend

```
You: I need a Rust backend for a blog

Goose: I'll help you set up a blog backend using the auto-rust tools.

You: Start by setting up the project

Goose: [calls setup_rust_project]

You: Now generate SQL for posts with title, content, author_id, and created_at

Goose: [calls generate_sql with description]

You: Add a GET endpoint for posts

Goose: [calls add_get_endpoint]
```

### Add Features to Existing Project

```
You: Add MinIO file upload support

Goose: [calls add_minio_integration]

You: Also add a Python microservice for image processing

Goose: [calls add_python_service]
```

## Files You Need to Know

- **`auto_rust_mcp.py`** - The MCP server (don't edit unless extending)
- **`auto-rust-recipe.yaml`** - Goose configuration
- **`venv/`** - Python dependencies (already set up)

## Troubleshooting One-Liners

```bash
# Test MCP server works
venv/bin/python auto_rust_mcp.py
# (Should show FastMCP banner, Ctrl+C to exit)

# Check Goose version
goose --version

# View Goose logs
tail -f ~/.config/goose/logs/*.log

# Rebuild Rust if needed
cargo build --release
```

## What If Something Breaks?

1. **Extension doesn't load**: Check paths in `auto-rust-recipe.yaml` are absolute
2. **Tools fail**: Check cargo is in PATH: `which cargo`
3. **Python errors**: Make sure using venv: `venv/bin/python`
4. **Timeout**: Increase `timeout: 120` in the recipe

## Want to Add More Tools?

Edit `auto_rust_mcp.py` and add:

```python
@mcp.tool()
def my_new_tool(param: Annotated[str, "Description"]) -> str:
    """What this tool does"""
    return run_auto_rust("new_command", custom_param=param)
```

Then restart Goose and it will see the new tool!

---

**Full docs**: See `GOOSE_SETUP.md` and `INTEGRATION_SUMMARY.md`
