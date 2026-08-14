mod add_compose;
use std::{env::current_dir, path::Path};
mod cicd;
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

use crate::cicd::add_git_acctions;
use crate::fix_structs::add_pub;
use crate::gen_sql_crate::gen_sql_crate;
use add_compose::add_compose;
use add_minio::add_minio;
use add_one_sql_funk::{add_one_post, add_one_sql_funk};
use add_python::add_python_func;
use add_react::create_react_app;
use boilerplate::{add_axum_end, add_top_boilerplate};
use gen_sql::gen_sql;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    what_to_make: String,
    #[arg(short, long, default_value_t = String::from("no_sql"))]
    sql: String,
    #[arg(short, long, default_value_t = String::from("no_table_name"))]
    dto_name: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let project_dir = current_dir().unwrap();
    println!("Project directory: {}", project_dir.display());

    match args.what_to_make {
        val if val == "setup" => match setup::setup() {
            Ok(_) => {
                println!("setup_res successful");
            }
            Err(e) => {
                println!("setup_res error: {}", e);
            }
        },
        val if val == "sql" => {
            // let mut sql_task = String::new();

            let mut sql_task = args.sql;

            let sql_task = if sql_task.trim().to_string().is_empty() {
                String::from(
                    "make a database to track infomation about hosts and renters for an airBnB like aplication. there are hosts that have a zip code, name, email, and password hash. there are also renters that have all the same colums expet the zip code.",
                )
            } else {
                sql_task.trim().to_string()
            };

            // Generate SQL and create necessary files
            match gen_sql(sql_task.clone(), false).await {
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
        var if var == "get_endpoint" => match add_one_sql_funk(args.dto_name) {
            Ok(_) => (),
            Err(e) => println!("sql gen error: {e}"),
        },
        var if var == "sql_crate" => match gen_sql_crate() {
            Ok(_) => (),
            Err(e) => print!("gen sql error : {e}"),
        },
        var if var == "post" => match add_one_post(args.dto_name) {
            Ok(_) => (),
            Err(e) => println!("post error: {e}"),
        },
        var if var == "minio" => {
            let path = project_dir.join("src/main.rs");
            println!("{}", path.display());
            match add_minio(&path) {
                Ok(_) => (),
                Err(e) => println!("minio error: {e}"),
            }
        }
        var if var == "pub_struct" => {
            add_pub(&project_dir.join("backend/src/models"))?;
        }
        // var if var == "cicd" => {
        //     add_git_acctions(&project_dir, &file_name)?;
        // }
        var if var == "h" => {
            println!(
                "valid options are: setup, sql, get_endpoint, sql_crate, post, minio, pub_struct"
            );
        }

        _ => println!(
            "valid options are: setup, sql, get_endpoint, sql_crate, post, minio, pub_struct"
        ),
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
