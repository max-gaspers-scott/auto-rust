use std::{
    env::current_dir,
    fs::{self, File},
    io::Write,
};

use dotenv::dotenv;
use reqwest::{
    blocking::get,
    header::{ACCEPT, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use std::env;

pub fn gen_sql(
    sql_task: String,
    get_sql: impl Fn(String) -> String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let project_dir = current_dir().unwrap();
    let migrations_dir = project_dir.join("migrations");
    // Create parent directories
    println!("Creating directory: {}", migrations_dir.display());
    fs::create_dir_all(&migrations_dir).map_err(|e| {
        eprintln!("Error creating directory: {}", e);
        e
    })?;
    let sql_path = migrations_dir.join("0001_data.sql");
    println!("Creating SQL file at: {}", sql_path.display());

    println!("made it to befor dotenv");
    let sql = get_sql(sql_task.to_string());
    println!("Generated SQL: {}", sql);
    // Create and write to the file
    println!("Creating file: {}", sql_path.display());
    let mut file = File::create(&sql_path).map_err(|e| {
        eprintln!("Error creating file: {}", e);
        e
    })?;

    file.write_all(sql.as_bytes()).map_err(|e| {
        eprintln!("Error writing to file: {}", e);
        e
    })?;
    Ok(())
}
