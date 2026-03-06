use file_ops::append_to_file;
use std::env;
use std::io;

use crate::add_fastapi::add_fastapi;

pub fn add_python_func(file_path: &std::path::Path) -> Result<(), io::Error> {
    let current_dir = env::current_dir()?;
    let fastapi_res = add_fastapi(&current_dir);

    match fastapi_res {
        Ok(_) => println!("added the fastapi folder "),
        Err(e) => eprintln!("error while adding the fastapi folder: {}", e),
    }

    let python_func = r###"


async fn python() -> Result<Json<Value>, (StatusCode, String)> {
    // Call the Python FastAPI service
    let client = reqwest::Client::new();
    let res = client
        .get("http://python:8003/chat")  // Use service name and correct port
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Request failed: {}", e)))?;

    if res.status().is_client_error() || res.status().is_server_error() {
        return Err((StatusCode::BAD_REQUEST, format!("Error from Python service: {}", res.status())));
    }

    let json_response: Value = res
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(json!({"payload": json_response})))
}

"###;
    append_to_file(file_path, python_func)
}
