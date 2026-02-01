mod add_compose;
mod add_fastapi;
mod add_functions;
mod add_minio;
mod add_object;
mod add_python;
mod add_react;
mod add_tests;
mod base_structs;
mod boilerplate;
mod gen_docker;
mod gen_examples;
mod gen_sql;
mod gen_toml;
// mod llm;
mod add_one_sql_funk;
mod gen_sql_crate;
mod schema;
mod sql_funcs;

mod setup;
use crate::gen_sql_crate::gen_sql_crate;
use add_compose::add_compose;
use add_fastapi::add_fastapi;
use add_minio::add_minio;
use add_one_sql_funk::add_one_sql_funk;
use add_python::add_python_func;
use add_react::create_react_app;
pub use base_structs::{Row, create_type_map};
use boilerplate::{add_axum_end, add_top_boilerplate};
use convert_case::{Case, Casing};
use gen_docker::gen_docker;
use gen_examples::gen_examples;
use gen_sql::gen_sql;
use gen_toml::gen_toml;
pub use schema::{Col, extract_column_info, extract_table_names, extract_table_schemas};
use serde::de::value::{self, Error};
pub use sql_funcs::add_basic_sql_funcs;
use sqlx::FromRow;
use std::collections::HashMap;
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufWriter};
use std::net::{SocketAddr, TcpListener};
use std::path;
use std::process::{Command, Output};

// This function is now in base_structs.rs
fn create_rows_from_sql(file_path: &std::path::Path) -> Result<Vec<Row>, io::Error> {
    let table_names = extract_table_names(&file_path.display().to_string())?;
    let schemas = extract_table_schemas(&file_path.display().to_string())?;
    let mut rows: Vec<Row> = Vec::new();

    if table_names.len() != schemas.len() {
        eprintln!("Warning: Number of table names and schemas do not match!");
    }

    for (table_name, schema) in table_names.iter().zip(schemas.iter()) {
        let cleaned_name = table_name
            .split('.')
            .last()
            .unwrap_or(&table_name)
            .trim_matches('"')
            .to_string();
        let cols = extract_column_info(schema);
        //let cols = c.into_iter().filter(|col| {
        //  !col.auto_gen
        //}).collect::<Vec<_>>();
        let row = Row {
            name: cleaned_name,
            cols,
        };
        rows.push(row);
    }

    Ok(rows)
}

// todo: kick off postgress
// https://users.rust-lang.org/t/how-to-execute-a-root-command-on-linux/50066/7
// docker run --name some-postgres -e POSTGRES_USER=dbuser -e POSTGRES_PASSWORD=p -e POSTGRES_DB=work -p 1111:5432 -d postgres
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut file_name = String::new();
    println!("Enter project name: ");
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim().to_string();

    let parent_dir = std::env::current_dir()?
        .parent()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Cannot get parent directory")
        })?
        .to_path_buf();

    let project_dir = parent_dir.join(&file_name);
    println!("Project directory: {}", project_dir.display());
    println!("Parent directory: {}", parent_dir.display());

    let setup_res = setup::setup(&parent_dir, &file_name);

    match setup_res {
        Ok(_) => {
            println!("setup_res successful");
        }
        Err(e) => {
            println!("setup_res error: {}", e);
        }
    }
    add_one_sql_funk();
    // Generate SQL and create necessary files
    let mut sql_task = String::new();
    println!(
        "Enter the specific task for the SQL database (e.g., 'make SQL to store users and their favored food'): "
    );
    io::stdin().read_line(&mut sql_task)?;
    let mut sql_task = sql_task.trim().to_string();
    if sql_task == "" {
        sql_task = "make a database to track infomation about hosts and renters for an airBnB like aplication. there are hosts that have a zip code, name, email, and password hash. there are also renters that have all the same colums expet the zip code.".to_string();
        println!("using default test string");
    }

    match gen_sql::gen_sql(project_dir.clone(), file_name.clone(), true).await {
        Ok(content) => {
            println!("Successfully generated SQL ({} bytes)", content.len());
        }
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to generate SQL: {}", e),
            ));
        }
    }

    // Process the generated SQL file
    let sql_path = project_dir.join("migrations/0001_data.sql");
    println!("Attempting to read SQL file from: {}", sql_path.display());

    // Verify file exists
    if !std::path::Path::new(&sql_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("SQL file does not exist at: {}", sql_path.display()),
        ));
    }

    let r = create_rows_from_sql(&sql_path);
    let rows = match r {
        Ok(rows) => {
            println!(
                "Successfully parsed {} table definitions from SQL",
                rows.len()
            );
            rows
        }
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Error parsing SQL file at {}: {}", sql_path.display(), e),
            ));
        }
    };

    // let crate_res = gen_sql_crate(&project_dir);
    //
    // match crate_res {
    //     Ok(res) => {
    //         println!("crate_res worked");
    //     }
    //     Err(e) => {
    //         println!("could not use the sql gen great: {}", e);
    //     }
    // }
    //
    let path = project_dir.join("src/main.rs");
    add_python_func(&path);
    let mut func_names = Vec::new();

    // TODO: rename, this creates select all, select one, and add functions.
    add_basic_sql_funcs(rows, &path, &mut func_names)?;
    println!("function names after basic sql are {:?}", func_names);
    // add_python_func(&path)?;
    // add_axum_end(func_names.clone(), &path)?;

    Ok(())
}

// need to:
// re-facter
// minio for more than just text
// use sql-gen crate
// get rid of port mapings (besided 8081) they are not needed, jsut use :minio or :backend
// ** curent code uses minio:9000 and but should change this
// ** https://gemini.google.com/app/61d9393cfe723e22?is_sa=1&is_sa=1&android-min-version=301356232&ios-min-version=322.0&campaign_id=bkws&utm_source=sem&utm_source=google&utm_medium=paid-media&utm_medium=cpc&utm_campaign=bkws&utm_campaign=2024enUS_gemfeb&pt=9008&mt=8&ct=p-growth-sem-bkws&gclsrc=aw.ds&gad_source=1&gad_campaignid=22908443171&gclid=Cj0KCQjw5c_FBhDJARIsAIcmHK8DwmYDLpVH8zs9IJmb2i1lSZtVT5NVUQvPOMa7tcObjfkuMQJdX3kaAsNBEALw_wcB

// CICD plan
// make a docker file that exposese port
// make docker compose yaml to start postgres (and volume), and rust (and exposse to internet)
//

// add ai to make desisions about what to add
// * test ollama based on videos
// * get function calling working
// * use funciton calling to call functions to generate code
// combin stuff with joins and filtering

// make call other arbitary apis like with requests.
// maybe function that takes in a url and schema struct and makes function that hits hits that url
//      with data in the structs format
//   would consiter this working when can hit open ai api tools

// at some point should ...
// should add RTC streams,and sockets (will help for streaming llm stuff)

// auto make unit tests for all functions

// add function to call ollama/apis  (can probably use comsom url in ollama_rs to hit open router endpoints)
// * maybe do langchain in another container that cals rust?
//  could be slow thought if using network between containers
// * or have langchain run in a proces kicked off my rust.
//  actor based model to comunicate between procesis

// call python code that writen in a python file (just in case)
