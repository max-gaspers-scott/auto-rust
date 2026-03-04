use io::Write;

use std::fs::OpenOptions;
use std::{fs, io};
pub fn add_one_sql_funk() -> Result<(), std::io::Error> {
    let mut sql_struct = String::new();
    println!("what table do you want to use"); // should give user list to pick from
    io::stdin()
        .read_line(&mut sql_struct)
        .expect("error reading from std in");
    sql_struct = sql_struct.trim_end().to_string();
    let file_path = format!("src/models/{}.rs", sql_struct.trim());
    println!("file name: {}", file_path);
    // read file
    let sql_bytes = fs::read(file_path).expect("that was not a valid file / sql struct");
    let sql = String::from_utf8(sql_bytes).expect("file contains invalid UTF-8");

    // split on lines and get third row (index 2)
    let lines: Vec<&str> = sql.lines().collect();
    let struct_type = if lines.len() > 2 {
        let third_row = lines[2];
        let words: Vec<&str> = third_row.split_whitespace().collect();
        if words.len() > 1 {
            words[1] // second word (index 1)
        } else {
            "no second word"
        }
    } else {
        "no third row"
    };

    // ask about what colums to return
    println!("enter colums do you want to be returned. seperte with spaces");
    let mut return_cols = String::new();
    io::stdin()
        .read_line(&mut return_cols)
        .expect("error reading from std in");
    return_cols = return_cols.trim_end().to_string();
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
    let rust = format!(
        r###"


#[derive(Debug, Deserialize)]
struct {query_struct_name} {{
    {match_col}: {struct_type},
}}

fn get_{return_cols}_{match_col}(match_val: Query<{struct_type}>) -> Json<Value> {{
    
    let query = format!(\"SELECT * FROM users WHERE user_id = $1\");
    let q = sqlx::query_as::<_, {sql_struct}>(&query).bind(match_val.{match_col}.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {{
        (StatusCode::INTERNAL_SERVER_ERROR, format!(\"Database err{{}}\", e))
    }})?;

    match elemint {{
        Some(elemint) => Ok(Json(json!({{
            "payload": {{
                elemint.{return_cols}
            }}
        }}))),
        None => Err((StatusCode::NOT_FOUND, "No record found with user_id = the value")),
    }}
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
