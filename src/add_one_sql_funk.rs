use convert_case::{Case, Casing};
use io::{BufWriter, Write};
use serde_urlencoded::to_string;
use std::io::prelude::*;
use std::path::PathBuf;

use std::fs::OpenOptions;
use std::path::{self, Path};
use std::{fmt::format, fs, io};
pub fn add_one_sql_funk() -> Result<(), std::io::Error> {
    // get file name as input
    let mut sql_struct = String::new();
    println!("what table do you want to use"); // should give user list to pick from
    io::stdin()
        .read_line(&mut sql_struct)
        .expect("error reading from std in");
    sql_struct = sql_struct.trim_end().to_string();
    let file_path = format!("src/models/{}.rs", sql_struct.trim());
    // read file
    let sql_bytes = fs::read(file_path).expect("that was not a valid file / sql struct");
    let sql = String::from_utf8(sql_bytes).expect("file contains invalid UTF-8");

    // split on lines and get third row (index 2)
    let mut lines: Vec<&str> = sql.lines().collect();
    let mut struct_type = if lines.len() > 2 {
        let third_row = lines[2];
        let words: Vec<&str> = third_row.split_whitespace().collect();
        if words.len() > 1 {
            words[1] // second word (index 1)
        } else {
            "no second word"
        }
    } else {
        "no third row"
    }
    .to_string();
    // comunity, make function
    struct_type.pop();

    println!("type is: {}", struct_type);
    // will change order but shouldent matter
    lines.swap_remove(lines.len() - 1);
    lines.swap_remove(0);
    lines.swap_remove(1);

    // ask about what colums to return
    println!("enter colums do you want to be returned. seperte with spaces");
    let mut return_cols = String::new();
    io::stdin()
        .read_line(&mut return_cols)
        .expect("error reading from std in");
    return_cols = return_cols.trim_end().to_string();
    let return_cols_commas = return_cols.replace(" ", ", ");
    // ask what colum to match on
    println!("what colum do you want to match (the select ___ part");
    let mut match_col = String::new();
    io::stdin()
        .read_line(&mut match_col)
        .expect("error readim from std in");

    // write a func to main.rs
    match_col = match_col.trim_end().to_string();
    let query_struct_name = format!("{match_col}Query").to_case(Case::Pascal);
    let payload: Vec<&str> = return_cols.split_whitespace().collect();
    let payload: String = payload
        .iter()
        .map(|name| format!("\t\"{name}\": elemint.{name},\n"))
        .collect::<String>()
        .trim_end_matches(", \n")
        .to_string();
    let retern_underscors = return_cols.replace(" ", "_");
    println!("{retern_underscors}");
    let sql_struct_captial = sql_struct.clone().to_case(Case::Pascal);

    let rust = format!(
        r###"


#[derive(sqlx::FromRow, Debug, Deserialize)]
struct {query_struct_name} {{
    {match_col}: {struct_type},
}}

// need to add ..
// mod models;
// use crate::models::User;

async fn get_{retern_underscors}_{match_col}(
        extract::State(pool): extract::State<PgPool>,
        match_val: Query<{query_struct_name}>, \\ may need to start with Query(params): ...
    ) -> Result<Json<Vec<{sql_struct_captial}>>, (StatusCode, String)> {{
    let mut query = format!("SELECT * FROM {sql_struct}s WHERE {match_col} = $1");

    let q = sqlx::query_as::<_, {sql_struct_captial}>(&query).bind(match_val.{match_col}.clone());

    let elemint = q.fetch_all(&pool).await.map_err(|e| {{
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{{}}", e))
    }})?;

    Ok(Json(elemint))
}}
"###
    );

    // for testing
    println!("rust: {}", rust);
    println!("query stuct name: {}", query_struct_name);

    let file_path = std::env::current_dir()?.join("src/main.rs");

    let mut file = OpenOptions::new().append(true).open(file_path.clone())?;
    // let mut file = BufWriter::new(file);

    let write_res = write!(&mut file, "{}", rust);

    match write_res {
        Ok(_) => {}
        Err(e) => println!(
            "an error writeing to {}: {}",
            file_path.clone().display(),
            e
        ),
    }

    Ok(())
}

pub fn add_one_post() -> Result<(), std::io::Error> {
    // get file name as input
    let mut sql_struct = String::new();
    println!("what table do you want to use"); // should give user list to pick from
    io::stdin()
        .read_line(&mut sql_struct)
        .expect("error reading from std in");
    sql_struct = sql_struct.trim_end().to_string();
    let file_path = format!("src/models/{}.rs", sql_struct.trim());
    // read file
    let sql_bytes = fs::read(file_path).expect("that was not a valid file / sql struct");
    let sql = String::from_utf8(sql_bytes).expect("file contains invalid UTF-8");

    let all_lines: Vec<&str> = sql.lines().collect();
    let len = all_lines.len();
    let lines = &all_lines[2..len - 2];
    let og_fields: Vec<String> = lines
        .iter()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut word = words[1].to_string();
            word.pop();
            word
        })
        .collect();
    let fields = &og_fields[1..];

    let sql_struct_captial = sql_struct.to_case(Case::Pascal);
    let mut instert_fields = String::new();
    // add every field in struct

    // change from hard coding
    for field in fields.clone() {
        instert_fields.push_str(&format!("{field}, "));
    }
    instert_fields.pop();
    instert_fields.pop();

    let mut bind_statment = String::new();
    for feild in fields {
        bind_statment.push_str(&format!("\n.bind(payload.{})", feild));
    }

    let mut doller_numbers = String::new();
    for i in 1..=fields.len() {
        doller_numbers.push_str(&format!("${i}, "));
    }
    doller_numbers.pop();
    doller_numbers.pop();

    let data_func = format!(
        r###"
        // db teble names have a s at the end that is removed in struct name
// you will need to add serde Deserialize and Deserialize to the structs
pub async fn post_{sql_struct}(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<{sql_struct_captial}>,
) -> Json<Value> {{
// change hardcoded number of values
    let query = "INSERT INTO {sql_struct}s ({instert_fields}) VALUES ({doller_numbers}) RETURNING *";

//// what is bound is wrong
    let q = sqlx::query_as::<_, {sql_struct_captial}>(&query){bind_statment};

    let result = q.fetch_one(&pool).await;

    match result {{
        Ok(value) => Json(json!({{"res": "success", "data": value}})),
        Err(e) => Json(json!({{"res": format!("error: {{}}", e)}}))
    }}
}}
"###
    );
    println!("post");

    let file_path = std::env::current_dir()?.join("src/main.rs");

    let mut file = OpenOptions::new().append(true).open(file_path.clone())?;
    // let mut file = BufWriter::new(file);

    let write_res = write!(&mut file, "{}", data_func);

    match write_res {
        Ok(_) => {}
        Err(e) => println!(
            "an error writeing to {}: {}",
            file_path.clone().display(),
            e
        ),
    };
    prepend_line_to_file(file_path.clone(), "mod models;");

    prepend_line_to_file(
        file_path,
        &format!("use crate::models::{};", sql_struct_captial),
    );

    Ok(())
}

fn prepend_line_to_file(path: PathBuf, line_to_add: &str) -> Result<(), std::io::Error> {
    let original_content = fs::read_to_string(path.clone())?;

    let new_content = format!("{}\n{}", line_to_add, original_content);

    fs::write(path, new_content)?;

    Ok(())
}
