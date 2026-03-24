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

    prepend_line_to_file(file_path.to_path_buf(), top_of_file)?;

    let funk_str = r###"

// add to top 



  let provider = StaticProvider::new("your-access-key", "your-secret-key", None);
  let minio = Minio::builder()
      .endpoint("localhost:9000")
      .provider(provider)
      .secure(false)
      .build()
      .unwrap();

async fn get_put_url(id: String) -> String {{
    let location = format!("media/chat-id/{{}}", id);
  minio
      .presigned_put_object(
          PresignedArgs::new("your-bucket", location)
              .expires(15 * 60),
      )
      .await?
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
