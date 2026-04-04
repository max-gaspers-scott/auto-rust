use file_ops::{append_to_file, prepend_line_to_file};
use std::io::{self};

pub fn add_minio(file_path: &std::path::Path) -> Result<(), io::Error> {
    // Ensure parent directories exist
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let top_of_file = r###"
    // minio stuff
  use minio_rsc::Minio;
  use minio_rsc::client::PresignedArgs;
  use minio_rsc::provider::StaticProvider;

  let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
  let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");
  let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT not set");

    "###;

    prepend_line_to_file(file_path, top_of_file)?;

    let funk_str = r###"

// add to top


fn build_minio_client(endpoint: &str) -> Minio {{
    let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
    let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");
    let secure = env::var("MINIO_SECURE").map(|v| v == "true").unwrap_or(false);
    let provider = StaticProvider::new(&access_key, &secret_key, None);
    Minio::builder()
        .endpoint(endpoint)
        .provider(provider)
        .secure(secure)
        .build()
        .unwrap()
}}

#[derive(Debug, Deserialize)]
struct UploadUrlQuery {
    file_extension: String,
}

#[derive(Debug, Deserialize)]
struct FetchUrlQuery {
    object_key: String,
}

async fn get_put_url(
    extract::State(pool): extract::State<PgPool>,
    Query(params): Query<UploadUrlQuery>,
) -> Json<Value> {{
    let object_key = format!("media/{{}.{{}}", Uuid::new_v4(), params.file_extension);
    let public_endpoint = env::var("MINIO_PUBLIC_ENDPOINT").expect("MINIO_PUBLIC_ENDPOINT not set");
    let minio = build_minio_client(&public_endpoint);
    match minio
        .presigned_put_object(PresignedArgs::new("bucket", &object_key).expires(15 * 60))
        .await
    {{
        Ok(url) => Json(json!({"status": "success", "upload_url": url, "object_key": object_key})),
        Err(e) => Json(json!({"status": "error", "error getting minio presigned url: ": e.to_string()})),
    }}
}}

async fn get_fetch_url(id: String) -> String {{
    let location = format!("media/chat-id/{{}}", id);
  minio
      .presigned_get_object(
          PresignedArgs::new("your-bucket", location)
              .expires(3600),
      )
      .await?
    }}

    "###;

    append_to_file(file_path, funk_str)
}
