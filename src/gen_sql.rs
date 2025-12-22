use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::{
    fs::{self, File},
    io::Write,
};

use dotenv::dotenv;

pub async fn gen_sql(
    project_dir: std::path::PathBuf,
    sql_task: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let api_key_name = "GEMINI_API_KEY";
    let api_key: String = match env::var(api_key_name) {
        Ok(val) => val.trim().to_string(),
        Err(e) => {
            println!("couldn't interpret {api_key_name}: {e}");
            format!("{}", e)
        }
    };

    let prompt = format!(
        r#"you are a postgresSQL database designer. Here is how you should write postgres SQL code to define a database.
    
    Tables should be defined with CREATE TABLE IF NOT EXISTS. 
    Only use these datatypes: 
    - BOOL, CHAR, SMALLINT, SMALLSERIAL, INT2, INT, SERIAL, INT4, BIGINT, 
    - BIGSERIAL, INT8, REAL, FLOAT4, DOUBLE PRECISION, FLOAT8, VARCHAR, 
    - CHAR(N), TEXT, NAME, CITEXT, BYTEA, VOID, INTERVAL, 
    - INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, 
    - TIMESTAMPTZ, TIMESTAMP, DATE, TIME, TIMETZ, 
    - UUID, INET, CIDR, MACADDR, BIT, VARBIT, JSON, JSONB

    Rules:
    - Use UNIQUE where necessary (inline, not at the bottom of the table)
    - Use gen_random_uuid() when using UUIDs
    - Don't use NUMERIC, instead use INT or FLOAT
    - Don't use table names like `public.\"user\"`
    - All tables should have a UUID primary key that auto-increments
    - Don't use any comments
    - Output only the sql code, nothing else.

    Example:
    If I say "define a postgresSQL database that stores work sessions for users. 
    Each user has a start time, duration, break time, and a user. Each user has an email and a name. 
    Each work session has exactly one user and each user can have many work sessions."

    You should output:
    
    CREATE TABLE IF NOT EXISTS users (
        user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email VARCHAR(255) UNIQUE NOT NULL,
        name VARCHAR(255)
    );

    CREATE TABLE IF NOT EXISTS work_sessions (
        work_session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(user_id),
        start_time TIMESTAMPTZ NOT NULL,
        duration_seconds INT NOT NULL,
        break_duration_seconds INT NOT NULL DEFAULT 0
    );

    Example 2:
    if i say "define a postgresSQL database that stores users and runs. 
    a user has a name, email, and favoret shoe. 
    a run has a user, and started at date/time, and distance and a duration. 
    each run should have exactly one user, but a user can have many runs. 
    output only the sql code, nothing else."
    
    you should output:
    
    CREATE TABLE IF NOT EXISTS users (
        user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email VARCHAR(255) UNIQUE NOT NULL,
        name VARCHAR(255)
    );

    CREATE TABLE IF NOT EXISTS runs (
        run_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(user_id),
        start_time TIMESTAMPTZ NOT NULL,
        distance_km FLOAT NOT NULL,
        duration_seconds INT NOT NULL
    );

    

    now the teask is: {}"#,
        sql_task
    );

    #[derive(Deserialize, Debug, Serialize)]
    struct Part {
        text: String,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct Content {
        parts: Vec<Part>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct Candidate {
        content: ContentResponse,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct ContentResponse {
        parts: Vec<PartResponse>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct PartResponse {
        text: String,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct GenerateContentResponse {
        contents: Vec<Content>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct GeminiRespons {
        candidates: Vec<Candidate>,
    }

    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key.to_string(),
        Err(e) => e.to_string(),
    };
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    // 3. Construct the Request Body using the Serde structs
    let request_body = GenerateContentResponse {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let mut sql = "defalut sql, if you see this text somthing has progably gone wrong".to_string();
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        // reqwest::Client::post() automatically uses the body's Serialize implementation
        // and sets the Content-Length header when sending the request body.
        .json(&request_body)
        .send()
        .await?;

    let sql = if response.status().is_success() {
        // Deserialize the JSON response into our Rust struct
        let json_response: GeminiRespons = response.json().await?;

        // TODO: should not return "" insted do better error handeling
        // program should not continue with empty string is somthing goes wrong at this step
        if let Some(candidate) = json_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                part.text.to_string()
            } else {
                println!("could not get part.text from api");
                "".to_string()
            }
        } else {
            println!("Response was successful but had no candidates.");
            "".to_string()
        }
    } else {
        eprintln!("\n❌ API Request Failed!");
        eprintln!("Status: {}", response.status());
        eprintln!("Body: {}", response.text().await?);
        "".to_string()
    };

    println!("Generated SQL: {}", sql);

    let migrations_dir = project_dir.join("migrations");
    let sql_path = migrations_dir.join("0001_data.sql");

    println!("Creating SQL file at: {}", sql_path.display());

    // Create parent directories
    println!("Creating directory: {}", migrations_dir.display());
    fs::create_dir_all(&migrations_dir).map_err(|e| {
        eprintln!("Error creating directory: {}", e);
        e
    })?;

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
    Ok("success".to_string())
}
