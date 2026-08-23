# Building and Using CloudWolf

This project bundles the `auto-rust` scaffolding tools directly into a modified
build of the `goose` AI agent, producing one self-contained `cloudwolf` binary.
The binary always routes model calls through the mgs proxy
(`https://mgs-proxy.team-stingray.com`) for Gemini, so there is no provider
config for the end user to set up.

CloudWolf keeps its state completely separate from an upstream `goose` install:
config lives in `~/.config/cloudwolf/`, data in `~/.local/share/cloudwolf/`, and
secrets under a `cloudwolf` system-keyring service. You can have both installed
side by side without them affecting each other.

## Prerequisites

- Rust toolchain (stable). Install via [rustup](https://rustup.rs) if needed.
- A C toolchain / linker (`cc`), which ships with most systems.

You do **not** need `cmake`. `cmake` is only required by goose's optional
`local-inference` feature (local llama.cpp models), which this integration does
not use. The build command below disables it.

## Build

From the repository root:

```bash
cd goose
cargo build -p goose-cli --bin cloudwolf --no-default-features --features portable-default
```

The compiled binary is written to:

```
goose/target/debug/cloudwolf
```

For an optimized release build, add `--release`; the binary then lands in
`goose/target/release/cloudwolf`.

```bash
cargo build -p goose-cli --bin cloudwolf --release --no-default-features --features portable-default
```

### Why these flags

- `--no-default-features` drops goose's default feature set, which includes
  `local-inference` (pulls in `llama-cpp-sys` and requires `cmake`).
- `--features portable-default` re-enables the parts we want without local
  inference: `rustls-tls`, `aws-providers`, `telemetry`, `otel`, and `tui`.

## Install

Copy the binary somewhere on your `PATH`, e.g.:

```bash
install -m755 goose/target/release/cloudwolf ~/.local/bin/cloudwolf
```

Confirm it is picked up:

```bash
cloudwolf --version
```

## First-time use

### 1. Log in

The proxy requires a fresh JWT. Log in once; the token is saved to
`~/.config/mgs-cli/token` and picked up automatically on every subsequent run.

```bash
cloudwolf login --email <your-email> --password <your-password>
```

There is nothing else to configure: the provider defaults to `openai`
(OpenAI-compatible) pointed at the proxy, and the model defaults to
`gemini-2.5-flash`. If `OPENAI_API_KEY` is unset, the saved JWT is used
automatically.

### 2. Start a session

```bash
cloudwolf session
```

Then just describe what you want to build in natural language. The embedded
`autorust` extension is enabled automatically and exposes these tools to the
agent:

- `setup_rust_project` — create the backend/frontend/docker skeleton
- `generate_sql` — write `migrations/0001_data.sql` from a description
- `make_structs_public` — after models are generated from the schema
- `add_get_endpoint` / `add_post_endpoint` — append handlers to `main.rs`

All tools operate inside the current working directory, so run `cloudwolf` from
the directory where you want the project scaffolded.

### Typical flow

1. `setup_rust_project` — scaffold the project.
2. `generate_sql` — describe your data model; migrations are generated.
3. `make_structs_public` — expose the generated model structs.
4. `add_get_endpoint` / `add_post_endpoint` — add API handlers.

## Verifying the embedded tools (optional)

You can confirm the `autorust` MCP server starts and lists its tools without
launching a full session:

```bash
printf '%s\n%s\n%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}' \
  '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  | cloudwolf mcp autorust
```

You should see `setup_rust_project`, `generate_sql`, `make_structs_public`,
`add_get_endpoint`, and `add_post_endpoint` in the response.

## Troubleshooting

- **`is cmake not installed?`** — You built with default features. Re-run with
  `--no-default-features --features portable-default` as shown above.
- **Auth errors when running a session** — Your JWT may be expired. Run
  `cloudwolf login` again to refresh `~/.config/mgs-cli/token`.
- **Old `goose` config interfering** — CloudWolf reads `~/.config/cloudwolf/`,
  not `~/.config/goose/`, so a pre-existing goose config cannot affect it.
