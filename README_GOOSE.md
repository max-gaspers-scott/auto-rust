# Auto-Rust Goose AI Integration

This project now provides **custom tools for Goose AI agent** to scaffold Rust web projects with Axum, PostgreSQL, Docker, and more.

## 🎯 What This Does

Goose can now use your Rust project generation tools to:
- ✅ Initialize complete Rust web projects with backend, frontend, and Docker
- ✅ Generate SQL schemas from natural language descriptions
- ✅ Create REST API endpoints (GET/POST) for database models
- ✅ Add MinIO object storage integration
- ✅ Add Python FastAPI microservices
- ✅ Generate database operation crates
- ✅ Make structs public for external access

## 🚀 Quick Start

### 1. Validate Setup

```bash
./validate_setup.sh
```

This checks all dependencies and configurations.

### 2. Test It

```bash
./test_in_project.sh
```

This creates a test directory and shows you how to use Goose.

### 3. Use From Any Project

```bash
cd /path/to/your-project
goose run --recipe /home/mgs/auto-rust-ai-agent/auto-rust-recipe.yaml -s
```

Then ask Goose things like:
- "What auto-rust tools do you have?"
- "Set up a new Rust web project"
- "Generate SQL for a users table with email and password"
- "Add a GET endpoint for the posts table"

## 📚 Documentation

- **`QUICK_START.md`** - Quick reference and examples (⭐ Start here!)
- **`INTEGRATION_SUMMARY.md`** - Complete technical overview
- **`GOOSE_SETUP.md`** - Detailed setup instructions

## 🛠️ Available Tools

| Tool | Description |
|------|-------------|
| `setup_rust_project` | Initialize new Rust backend with boilerplate |
| `generate_sql` | Generate PostgreSQL migrations from descriptions |
| `add_get_endpoint` | Generate GET endpoint for a model |
| `add_post_endpoint` | Generate POST endpoint for a model |
| `add_minio_integration` | Add MinIO object storage |
| `add_python_service` | Add FastAPI microservice |
| `generate_sql_crate` | Create SQL operations crate |
| `make_structs_public` | Make all model structs public |

## 🔧 How It Works

```
┌─────────────┐
│ User asks   │
│   Goose     │
└──────┬──────┘
       │
       ▼
┌─────────────────────┐
│ Goose sees tools    │
│ from MCP server     │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ auto_rust_mcp.py    │
│ translates request  │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ cargo run --        │
│ your Rust tools     │
└─────────────────────┘
```

## 📁 Files Overview

```
auto-rust-ai-agent/
├── auto_rust_mcp.py           # MCP server (Goose integration)
├── auto-rust-recipe.yaml      # Goose configuration
├── venv/                      # Python dependencies
├── validate_setup.sh          # Check everything works
├── test_in_project.sh         # Easy testing
├── QUICK_START.md            # ⭐ Quick reference
├── INTEGRATION_SUMMARY.md     # Technical details
├── GOOSE_SETUP.md            # Setup guide
└── src/                       # Your Rust tools
    ├── gen_toml.rs
    ├── gen_sql.rs
    ├── add_minio.rs
    └── ...
```

## ⚠️ Known Issues

1. **Cargo.toml has invalid edition** - `edition = "2024"` should be `"2021"`
   ```bash
   # Fix it:
   sed -i 's/edition = "2024"/edition = "2021"/' Cargo.toml
   ```

2. **I did NOT modify or test your Rust tools** - They work as-is, bugs and all
   - The integration exposes whatever the tools currently do
   - You may want to test them individually first

## 🧪 Example Session

```bash
$ cd ~/my-new-project
$ goose run --recipe /home/mgs/auto-rust-ai-agent/auto-rust-recipe.yaml -s

You: I need to create a blog backend with Rust

Goose: I can help you with that using the auto-rust tools. Let me start by 
       setting up the project structure.
       
       [Calls setup_rust_project]
       
       ✓ Created backend directory with Axum boilerplate
       ✓ Added Dockerfile and docker-compose.yaml
       ✓ Created React frontend
       
You: Now add a posts table with title, content, author_id, and created_at

Goose: [Calls generate_sql with description]
       
       ✓ Generated SQL migration file
       
You: Add a GET endpoint for posts

Goose: [Calls add_get_endpoint with dto_name="posts"]
       
       ✓ Added GET /posts endpoint
```

## 🔍 Troubleshooting

Run the validator first:
```bash
./validate_setup.sh
```

Common issues:
- **Tools fail**: Check `cargo` is in PATH
- **Extension won't load**: Check logs in `~/.config/goose/logs/`
- **Timeout**: Increase `timeout: 120` in `auto-rust-recipe.yaml`

## 🎓 Learn More

- [Goose Documentation](https://github.com/block/goose)
- [MCP Protocol](https://modelcontextprotocol.io/)
- [FastMCP](https://gofastmcp.com/)

## 💡 Tips

1. **Start Simple**: Try `show_help` tool first to see what's available
2. **Be Specific**: Tell Goose exactly what database fields you want
3. **Check Output**: Always review the generated code
4. **Iterate**: Ask Goose to modify or add to what it created

---

**Ready to test?** Run `./test_in_project.sh` to get started!
