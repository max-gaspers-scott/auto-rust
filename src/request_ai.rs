use std::fs;
use std::path::PathBuf;
pub fn ai_test(user_reqwest: String) -> String {
    r#"
     CREATE TABLE IF NOT EXISTS users (
         user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
         username VARCHAR(255) UNIQUE NOT NULL,
         email VARCHAR(255) UNIQUE
     );

     CREATE TABLE IF NOT EXISTS messages (
         message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
         sender_id UUID NOT NULL REFERENCES users(user_id),
         recipiant_id UUID NOT NULL REFERENCES users(user_id),
         content TEXT NOT NULL,
         sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
     );"#
    .to_string()
}
use dotenv::dotenv;
use reqwest::{
    blocking::get,
    header::{ACCEPT, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use std::env;

//TODO: test
pub fn gemini_for_sql(user_reqwest: String) -> String {
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

     Rules:
     - Use UNIQUE where necessary (inline, not at the bottom of the table)
     - Use gen_random_uuid() when using UUIDs
     - Don't use table names like `public.\"user\"`
     - All tables should have a UUID primary key
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
        user_reqwest
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

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        // reqwest::Client::post() automatically uses the body's Serialize implementation
        // and sets the Content-Length header when sending the request body.
        .json(&request_body)
        .send()
        .unwrap();

    if response.status().is_success() {
        // Deserialize the JSON response into our Rust struct
        let json_response: GeminiRespons = response.json().unwrap();

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
        eprintln!("Body: {}", response.text().unwrap());
        "".to_string()
    }
}

// ---------------------------------------------------------------------------
// Request / response types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct SignupPayload {
    username: String,
    email: String,
    password: String,
    is_pro: bool,
}

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    res: String,
    token: Option<String>,
}

/// OpenAI-compatible request body.
#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatPayload {
    model: String,
    messages: Vec<ChatMessage>,
}

/// OpenAI-compatible response — we only need the fields we use.
#[derive(Deserialize)]
struct ChatCompletionMessage {
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

// pub fn mgs_proxy(user_reqwest: String) -> String {
//     let client = reqwest::blocking::Client::new();
//     let payload = ChatPayload {
//         model: "gemini-2.5-flash".to_string(),
//         messages: vec![ChatMessage {
//             role: "user".to_string(),
//             content: user_reqwest,
//         }],
//     };
//
//     let config_dir = dirs::config_dir().context("could not determine config directory")?;
//     let dir = config_dir.join("auto-rust-cli");
//     fs::create_dir_all(&dir).context("could not create config directory")?;
//     let path = dir.join("token");
//     let token = fs::read_to_string(&path).context("no saved token — run `mgs login` first");
//
//     let api_url = "https://localhost:8081";
//     let resp = client
//         .post(format!("{api_url}/v1/chat/completions"))
//         .bearer_auth(&token)
//         .json(&payload)
//         .send()
//         .context("could not reach the server")?;
//
//     let status = resp.status();
//
//     if status == reqwest::StatusCode::UNAUTHORIZED {
//         panic!("token expired or invalid — run `mgs login` again");
//     }
//
//     if !status.is_success() {
//         let body: serde_json::Value = resp.json().unwrap_or_default();
//         let msg = body["error"]["message"].as_str().unwrap_or("unknown error");
//         panic!("chat failed (HTTP {}): {}", status, msg);
//     }
//
//     let body: ChatCompletionResponse = resp.json().context("server returned invalid JSON")?;
//     let reply = body
//         .choices
//         .into_iter()
//         .next()
//         .map(|c| c.message.content)
//         .unwrap_or_else(|| "(no response)".to_string());
//
//     println!("\nGemini: {}", reply);
//     reply
// }

pub fn login(email: String, password: String) {
    let client = reqwest::blocking::Client::new();

    let payload = LoginPayload { email, password };

    let api_url = "https://localhost:8081";
    let resp = client
        .post(format!("{api_url}/api/login"))
        .json(&payload)
        .send()
        .unwrap();
    if status.is_success() {
        if let Some(token) = body.token {
            save_token(&token).unwrap();
            let path = token_path().unwrap();
            println!("✓ Logged in. Token saved to {}", path.display());
        } else {
            panic!("login failed: {}", body.res);
        }
    } else {
        panic!("login failed (HTTP {}): {}", status, body.res);
    }
}

// ======================
//     token helpers
// ======================

fn token_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()?;
    let dir = config_dir.join("mgs-cli");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("token"))
}

fn load_token() -> Result<String> {
    let path = token_path()?;
    fs::read_to_string(&path)
}

fn save_token(token: &str) -> Result<()> {
    let path = token_path()?;
    fs::write(&path, token).unwrap();
    // Restrict permissions to owner-only on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}
