use convert_case::{Case, Casing};

use crate::models::handeler_meta_data::*;

use file_ops::{append_to_file, prepend_line_to_file};
use std::env::current_dir;
use std::path::Path;

struct SqlMetadata {
    sql_struct: String,
    capitalized_struct: String,
    sql: String, // should be path buf??
}

fn get_sql_metadata(dto_name: String) -> SqlMetadata {
    let sql_struct = dto_name.trim_end().to_string();
    let capitalized_struct = sql_struct.to_case(Case::Pascal);
    let file_path = format!("backend/src/models/{}.rs", sql_struct.trim());
    // read file
    let path_type = Path::new(&file_path);
    let temp = path_type.display();

    let sql = fs::read_to_string(path_type).expect("that was not a valid file / sql struct");

    SqlMetadata {
        sql_struct,
        capitalized_struct,
        sql,
    }
}

use std::{fs, io};

pub fn add_one_sql_funk(
    handelr_data: HandelerMetaData,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sql_metadata = get_sql_metadata(handelr_data.name);
    // split on lines and get third row (index 2)
    let lines: Vec<&str> = sql_metadata.sql.lines().collect();
    let struct_type = if lines.len() > 2 {
        let third_row = lines[2];
        let words: Vec<&str> = third_row.split_whitespace().collect();
        if words.len() > 2 {
            let mut word = words[2].to_string();
            word.pop();
            word
        } else {
            String::from("no second word")
        }
    } else {
        String::from("no third row")
    };

    // ask about what colums to return
    let return_cols = handelr_data.fields_to_retrun.trim_end().to_string();
    let return_underscors = return_cols.replace(" ", "_");
    // ask what colum to match on
    println!("what colum do you want to match (the select ___ part");
    let mut match_col = String::new();
    io::stdin()
        .read_line(&mut match_col)
        .expect("error readim from std in");

    // write a func to main.rs
    println!("type is: {}", struct_type);
    match_col = match_col.trim_end().to_string();
    let query_struct_name = format!("{match_col}_query");
    let mut payload = String::new();
    for e in return_cols.split_whitespace() {
        payload.push_str(&format!("\"{e}\": elemint.{e},\n"));
    }
    let capitalized = sql_metadata.capitalized_struct;
    let sql_struct = sql_metadata.sql_struct;
    let rust = format!(
        r###"


#[derive(Debug, Deserialize)]
struct {query_struct_name} {{
    {match_col}: {struct_type},
}}

async fn get_{return_underscors}_{match_col}(
    match_val: Query<{query_struct_name}>,
    extract::State(pool): extract::State<PgPool>,
) -> Json<Value> {{
    let query = format!("SELECT * FROM {sql_struct} WHERE {match_col} = $1");
    let q = sqlx::query_as::<_, {capitalized}>(&query).bind(match_val.{match_col}.clone());

    let elemint = q.fetch_optional(&pool).await;



    match elemint {{
        Ok(Some(elemint)) => Json(json!({{
            "status": "success",
            "payload": {{
            {payload}
            }}
        }})),
        Ok(None) => Json(json!({{
            "status": "error",
            "error": "User not found"
        }})),
        _ => Json(json!({{
            "status": "error",
            "error": "User not found"
        }})),
    }}
}}
"###
    );
    // for testing
    println!("rust: {}", rust);
    println!("query stuct name: {}", query_struct_name);

    let file_path = current_dir()?.join("backend/src/main.rs");
    append_to_file(&file_path, &rust)?;

    Ok(())
}

pub fn add_one_post(dto_name: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sql_metadata = get_sql_metadata(dto_name);
    let all_lines: Vec<&str> = sql_metadata.sql.lines().collect();
    let len = all_lines.len();
    if len < 4 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Model file has fewer than 4 lines",
        ))?;
    }

    let lines = &all_lines[2..len - 1];
    let og_fields: Vec<String> = lines
        .iter()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() < 2 {
                // Handle lines without enough tokens
                return String::new();
            }
            let mut word = words[1].to_string();
            word.pop();
            word
        })
        .filter(|s| !s.is_empty())
        .collect();
    let fields = &og_fields[1..];

    let sql_struct_captial = sql_metadata.capitalized_struct;
    let mut instert_fields = String::new();

    // change from hard coding
    for field in fields {
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

    let sql_struct = sql_metadata.sql_struct;
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
    let file_path = std::env::current_dir()?.join("backend/src/main.rs");

    append_to_file(&file_path, &data_func)?;
    prepend_line_to_file(&file_path, "mod models;")?;

    prepend_line_to_file(
        &file_path,
        &format!("use crate::models::{};", sql_struct_captial),
    )?;

    Ok(())
}
