use file_ops::append_to_file;

pub fn gen_toml(
    project_dir: &std::path::Path,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let deps = "
    axum = { version = \"0.7\", features = [\"macros\"] }
tokio = { version = \"1\", features = [\"full\", \"time\"] }
serde = { version = \"1\", features = [\"derive\"] }
serde_json = \"1\"
sqlx = { version = \"0.7\", features = [\"runtime-tokio-rustls\", \"postgres\", \"chrono\", \"uuid\"] } # Added \"uuid\" feature as it's often used with database interactions.
dotenv = \"0.15\" # Useful for loading environment variables like your database URL
chrono = { version = \"0.4\", features = [\"serde\"] } # For Utc
uuid = { version = \"1\", features = [\"serde\", \"v4\"] } # For UUID generation and serialization
tempfile = \"3.3\"
anyhow = \"1.0\"
minio-rsc = \"0.2.6\"
reqwest = { version = \"0.11\", features = [\"json\"] }
tower-http = { version = \"0.5\", features = [\"cors\", \"fs\"] } # For CorsLayer
tower = \"0.5.2\"

    ";

    match append_to_file(&project_dir.join("Cargo.toml"), deps) {
        Ok(_) => (),
        Err(e) => println!("toml error: {e}"),
    }
    Ok(deps.to_string())
}
