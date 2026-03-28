mod add_compose;
mod fix_structs;
mod gen_docker;
use clap::Parser;
mod add_fastapi;
mod add_minio;
mod add_python;
mod add_react;
mod boilerplate;
mod gen_sql;
mod gen_toml;
// mod llm;
mod add_one_sql_funk;
mod gen_sql_crate;

mod setup;

use crate::fix_structs::add_pub;
use crate::gen_sql_crate::gen_sql_crate;
use add_compose::add_compose;
use add_minio::add_minio;
use add_one_sql_funk::{add_one_post, add_one_sql_funk};
use add_python::add_python_func;
use add_react::create_react_app;
// pub use base_structs::{Row, create_type_map};
use boilerplate::{add_axum_end, add_top_boilerplate};
use gen_sql::gen_sql;
use std::io;

// This function is now in base_structs.rs

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    what_to_make: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut file_name = String::new();
    println!("Enter project name: ");
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim().to_string();
    let file_name = if file_name.is_empty() {
        std::env::current_dir()?.display().to_string()
    } else {
        file_name
    };

    let parent_dir = std::env::current_dir()?
        .parent()
        .ok_or_else(|| std::io::Error::other("Cannot get parent directory"))?
        .to_path_buf();

    let project_dir = parent_dir.join(&file_name);
    println!("Project directory: {}", project_dir.display());
    println!("Parent directory: {}", parent_dir.display());
    let args = Args::parse();

    match args.what_to_make {
        val if val == "setup" => match setup::setup(&parent_dir, &file_name) {
            Ok(_) => {
                println!("setup_res successful");
            }
            Err(e) => {
                println!("setup_res error: {}", e);
            }
        },
        val if val == "sql" => {
            let mut sql_task = String::new();
            println!(
                "Enter the specific task for the SQL database (e.g., 'make SQL to store users and their favored food'): "
            );
            io::stdin().read_line(&mut sql_task)?;

            let sql_task = if sql_task.trim().to_string().is_empty() {
                String::from(
                    "make a database to track infomation about hosts and renters for an airBnB like aplication. there are hosts that have a zip code, name, email, and password hash. there are also renters that have all the same colums expet the zip code.",
                )
            } else {
                sql_task.trim().to_string()
            };

            // Generate SQL and create necessary files
            match gen_sql(project_dir.clone(), sql_task.clone(), false).await {
                Ok(content) => {
                    println!("Successfully generated SQL ({} bytes)", content.len());
                }
                Err(e) => {
                    println!("sql error: {e}");
                }
            }
        }
        var if var == "python" => {
            let path = project_dir.join("src/main.rs");
            match add_python_func(&path) {
                Ok(_) => (),
                Err(e) => print!("python error: {e}"),
            }
        }
        var if var == "one_sql" => match add_one_sql_funk() {
            Ok(_) => (),
            Err(e) => println!("sql gen error: {e}"),
        },
        var if var == "sql_crate" => match gen_sql_crate(&project_dir) {
            Ok(_) => (),
            Err(e) => print!("gen sql error : {e}"),
        },
        var if var == "post" => match add_one_post() {
            Ok(_) => (),
            Err(e) => println!("post error: {e}"),
        },
        var if var == "minio" => {
            let path = project_dir.join("src/main.rs");
            match add_minio(&path) {
                Ok(_) => (),
                Err(e) => println!("minio error: {e}"),
            }
        }
        var if var == "pub_struct" => {
            add_pub(&project_dir.join("src/models"))?;
        }
        _ => println!("valid options are: setup, sql, one_sql, sql_crate, post, minio"),
    }
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
