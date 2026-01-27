
use axum::{                                                                                                                                                                      
    extract::{self, Path, Query},  
    routing::{get, post},                                                                                                                                                        
    Json, Router,                        
};       
use minio_rsc::{Minio, provider::StaticProvider, client::PresignedArgs};
use serde::{Deserialize, Serialize};                                                                                                                                                          
use serde_json::{json, Value};                                                                                                                                                  
use sqlx::PgPool;                                                                                                                                                               
use sqlx::{postgres::PgPoolOptions, prelude::FromRow};                                                                                                                           
use std::env;                                                                                                                                                                    
use std::net::SocketAddr;                                                                                                                                                        
use std::result::Result;                                                                                                                                                         
use std::sync::Arc;                                                                                                                                                              
use axum::http::StatusCode;                  
use sqlx::types::chrono::Utc; 
use std::collections::HashMap;
use tower_http::cors::{AllowOrigin, CorsLayer};
use axum::http::Method;
use reqwest;

use axum::response::{Html, IntoResponse};
use tower::service_fn;
use tower_http::services::ServeDir;


#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Users {
    user_id: Option<uuid::Uuid>,
    username: String,
    email: String,
    display_name: String,
    created_at: Option<chrono::DateTime<Utc>>,
}

pub async fn add_users(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Users>,
) -> Json<Value> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_add_users(extract::State(pool), Json(payload)).await;
    result
}

pub async fn data_add_users(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Users>,
) -> Json<Value> {
    let query = "INSERT INTO users (username, email, display_name) VALUES ($1, $2, $3) RETURNING *";
    
    let q = sqlx::query_as::<_, Users>(&query)
		.bind(payload.username)
		.bind(payload.email)
		.bind(payload.display_name);
    
    let result = q.fetch_one(&pool).await;

    match result {
        Ok(value) => Json(json!({"res": "success", "data": value})),
        Err(e) => Json(json!({"res": format!("error: {}", e)}))
    }
}


#[derive(Deserialize)]
struct usersQueryParams {
    order_by: Option<String>,
    direction: Option<String>, // "asc" or "desc"
    #[serde(flatten)]
    filters: HashMap<String, String>,
}


pub async fn get_users(
    extract::State(pool): extract::State<PgPool>,
    Query(query_params): Query<usersQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_users(extract::State(pool), axum::extract::Query(query_params)).await;
    result
}



pub async fn data_get_users(
    extract::State(pool): extract::State<PgPool>,
    query_params: axum::extract::Query<usersQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut query = "SELECT * FROM users".to_owned();
    let mut sql_params: Vec<String> = Vec::new();
    let mut param_index = 1;
    
    // Handle filters
    if !query_params.filters.is_empty() {
        let mut where_conditions: Vec<String> = Vec::new();
        
        for (field, value) in &query_params.filters {
            // Skip ordering parameters
            if field == "order_by" || field == "direction" {
                continue;
            }
            
            // Validate field name to prevent SQL injection
            if field.chars().all(|c| c.is_alphanumeric() || c == '_') {
                where_conditions.push(format!("{} = ${}", field, param_index));
                sql_params.push(value.clone());
                param_index += 1;
            } else {
                return Err((StatusCode::BAD_REQUEST, format!("Invalid field name: {}", field)));
            }
        }
        
        if !where_conditions.is_empty() {
            query.push_str(&(" WHERE ".to_owned() + &where_conditions.join(" AND ")));
        }
    }
    
    // Validate and apply ordering if provided
    if let Some(order_by) = &query_params.order_by {
        // Validate order_by column name to prevent SQL injection
        // Only allow alphanumeric characters and underscores
        if order_by.chars().all(|c| c.is_alphanumeric() || c == '_') {
            // Validate direction parameter
            let direction = match &query_params.direction {
                Some(dir) if dir.to_lowercase() == "desc" => "DESC",
                _ => "ASC",
            };
            
            query.push_str(&format!(" ORDER BY {} {}", *order_by, direction));
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid order_by parameter".to_string()));
        }
    }

    // Execute query with parameters
    let mut query_builder = sqlx::query_as::<_, Users>(&query);
    for param in &sql_params {
        query_builder = query_builder.bind(param);
    }

    let elemints: Vec<Users> = query_builder.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"user_id": elemint.user_id, 
	"username": elemint.username, 
	"email": elemint.email, 
	"display_name": elemint.display_name, 
	"created_at": elemint.created_at
        })
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}

#[derive(Debug, Deserialize)]
struct usersuser_idQuery {
    user_id: uuid::Uuid,
}

pub async fn get_one_usersuser_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersuser_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_usersuser_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_usersuser_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersuser_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM users WHERE user_id = $1");
    let q = sqlx::query_as::<_, Users>(&query).bind(match_val.user_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"user_id": elemint.user_id, 
	"username": elemint.username, 
	"email": elemint.email, 
	"display_name": elemint.display_name, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with user_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct usersusernameQuery {
    username: String,
}

pub async fn get_one_usersusername(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersusernameQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_usersusername(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_usersusername(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersusernameQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM users WHERE username = $1");
    let q = sqlx::query_as::<_, Users>(&query).bind(match_val.username.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"user_id": elemint.user_id, 
	"username": elemint.username, 
	"email": elemint.email, 
	"display_name": elemint.display_name, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with username = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct usersemailQuery {
    email: String,
}

pub async fn get_one_usersemail(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersemailQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_usersemail(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_usersemail(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersemailQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM users WHERE email = $1");
    let q = sqlx::query_as::<_, Users>(&query).bind(match_val.email.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"user_id": elemint.user_id, 
	"username": elemint.username, 
	"email": elemint.email, 
	"display_name": elemint.display_name, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with email = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct usersdisplay_nameQuery {
    display_name: String,
}

pub async fn get_one_usersdisplay_name(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersdisplay_nameQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_usersdisplay_name(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_usersdisplay_name(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<usersdisplay_nameQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM users WHERE display_name = $1");
    let q = sqlx::query_as::<_, Users>(&query).bind(match_val.display_name.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"user_id": elemint.user_id, 
	"username": elemint.username, 
	"email": elemint.email, 
	"display_name": elemint.display_name, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with display_name = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct userscreated_atQuery {
    created_at: chrono::DateTime<Utc>,
}

pub async fn get_one_userscreated_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<userscreated_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_userscreated_at(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_userscreated_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<userscreated_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM users WHERE created_at = $1");
    let q = sqlx::query_as::<_, Users>(&query).bind(match_val.created_at.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"user_id": elemint.user_id, 
	"username": elemint.username, 
	"email": elemint.email, 
	"display_name": elemint.display_name, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with created_at = the value"))),
    }
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Conversations {
    conversation_id: Option<uuid::Uuid>,
    name: String,
    is_group_chat: Option<bool>,
    created_at: Option<chrono::DateTime<Utc>>,
}

pub async fn add_conversations(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Conversations>,
) -> Json<Value> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_add_conversations(extract::State(pool), Json(payload)).await;
    result
}

pub async fn data_add_conversations(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Conversations>,
) -> Json<Value> {
    let query = "INSERT INTO conversations (name) VALUES ($1) RETURNING *";
    
    let q = sqlx::query_as::<_, Conversations>(&query)
		.bind(payload.name);
    
    let result = q.fetch_one(&pool).await;

    match result {
        Ok(value) => Json(json!({"res": "success", "data": value})),
        Err(e) => Json(json!({"res": format!("error: {}", e)}))
    }
}


#[derive(Deserialize)]
struct conversationsQueryParams {
    order_by: Option<String>,
    direction: Option<String>, // "asc" or "desc"
    #[serde(flatten)]
    filters: HashMap<String, String>,
}


pub async fn get_conversations(
    extract::State(pool): extract::State<PgPool>,
    Query(query_params): Query<conversationsQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_conversations(extract::State(pool), axum::extract::Query(query_params)).await;
    result
}



pub async fn data_get_conversations(
    extract::State(pool): extract::State<PgPool>,
    query_params: axum::extract::Query<conversationsQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut query = "SELECT * FROM conversations".to_owned();
    let mut sql_params: Vec<String> = Vec::new();
    let mut param_index = 1;
    
    // Handle filters
    if !query_params.filters.is_empty() {
        let mut where_conditions: Vec<String> = Vec::new();
        
        for (field, value) in &query_params.filters {
            // Skip ordering parameters
            if field == "order_by" || field == "direction" {
                continue;
            }
            
            // Validate field name to prevent SQL injection
            if field.chars().all(|c| c.is_alphanumeric() || c == '_') {
                where_conditions.push(format!("{} = ${}", field, param_index));
                sql_params.push(value.clone());
                param_index += 1;
            } else {
                return Err((StatusCode::BAD_REQUEST, format!("Invalid field name: {}", field)));
            }
        }
        
        if !where_conditions.is_empty() {
            query.push_str(&(" WHERE ".to_owned() + &where_conditions.join(" AND ")));
        }
    }
    
    // Validate and apply ordering if provided
    if let Some(order_by) = &query_params.order_by {
        // Validate order_by column name to prevent SQL injection
        // Only allow alphanumeric characters and underscores
        if order_by.chars().all(|c| c.is_alphanumeric() || c == '_') {
            // Validate direction parameter
            let direction = match &query_params.direction {
                Some(dir) if dir.to_lowercase() == "desc" => "DESC",
                _ => "ASC",
            };
            
            query.push_str(&format!(" ORDER BY {} {}", *order_by, direction));
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid order_by parameter".to_string()));
        }
    }

    // Execute query with parameters
    let mut query_builder = sqlx::query_as::<_, Conversations>(&query);
    for param in &sql_params {
        query_builder = query_builder.bind(param);
    }

    let elemints: Vec<Conversations> = query_builder.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"conversation_id": elemint.conversation_id, 
	"name": elemint.name, 
	"is_group_chat": elemint.is_group_chat, 
	"created_at": elemint.created_at
        })
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}

#[derive(Debug, Deserialize)]
struct conversationsconversation_idQuery {
    conversation_id: uuid::Uuid,
}

pub async fn get_one_conversationsconversation_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationsconversation_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversationsconversation_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversationsconversation_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationsconversation_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversations WHERE conversation_id = $1");
    let q = sqlx::query_as::<_, Conversations>(&query).bind(match_val.conversation_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"conversation_id": elemint.conversation_id, 
	"name": elemint.name, 
	"is_group_chat": elemint.is_group_chat, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with conversation_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct conversationsnameQuery {
    name: String,
}

pub async fn get_one_conversationsname(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationsnameQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversationsname(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversationsname(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationsnameQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversations WHERE name = $1");
    let q = sqlx::query_as::<_, Conversations>(&query).bind(match_val.name.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"conversation_id": elemint.conversation_id, 
	"name": elemint.name, 
	"is_group_chat": elemint.is_group_chat, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with name = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct conversationsis_group_chatQuery {
    is_group_chat: bool,
}

pub async fn get_one_conversationsis_group_chat(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationsis_group_chatQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversationsis_group_chat(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversationsis_group_chat(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationsis_group_chatQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversations WHERE is_group_chat = $1");
    let q = sqlx::query_as::<_, Conversations>(&query).bind(match_val.is_group_chat.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"conversation_id": elemint.conversation_id, 
	"name": elemint.name, 
	"is_group_chat": elemint.is_group_chat, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with is_group_chat = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct conversationscreated_atQuery {
    created_at: chrono::DateTime<Utc>,
}

pub async fn get_one_conversationscreated_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationscreated_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversationscreated_at(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversationscreated_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversationscreated_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversations WHERE created_at = $1");
    let q = sqlx::query_as::<_, Conversations>(&query).bind(match_val.created_at.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"conversation_id": elemint.conversation_id, 
	"name": elemint.name, 
	"is_group_chat": elemint.is_group_chat, 
	"created_at": elemint.created_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with created_at = the value"))),
    }
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
struct ConversationParticipants {
    participant_id: Option<uuid::Uuid>,
    conversation_id: uuid::Uuid,
    user_id: uuid::Uuid,
    joined_at: Option<chrono::DateTime<Utc>>,
}

pub async fn add_conversation_participants(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<ConversationParticipants>,
) -> Json<Value> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_add_conversation_participants(extract::State(pool), Json(payload)).await;
    result
}

pub async fn data_add_conversation_participants(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<ConversationParticipants>,
) -> Json<Value> {
    let query = "INSERT INTO conversation_participants (conversation_id, user_id) VALUES ($1, $2) RETURNING *";
    
    let q = sqlx::query_as::<_, ConversationParticipants>(&query)
		.bind(payload.conversation_id)
		.bind(payload.user_id);
    
    let result = q.fetch_one(&pool).await;

    match result {
        Ok(value) => Json(json!({"res": "success", "data": value})),
        Err(e) => Json(json!({"res": format!("error: {}", e)}))
    }
}


#[derive(Deserialize)]
struct conversation_participantsQueryParams {
    order_by: Option<String>,
    direction: Option<String>, // "asc" or "desc"
    #[serde(flatten)]
    filters: HashMap<String, String>,
}


pub async fn get_conversation_participants(
    extract::State(pool): extract::State<PgPool>,
    Query(query_params): Query<conversation_participantsQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_conversation_participants(extract::State(pool), axum::extract::Query(query_params)).await;
    result
}



pub async fn data_get_conversation_participants(
    extract::State(pool): extract::State<PgPool>,
    query_params: axum::extract::Query<conversation_participantsQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut query = "SELECT * FROM conversation_participants".to_owned();
    let mut sql_params: Vec<String> = Vec::new();
    let mut param_index = 1;
    
    // Handle filters
    if !query_params.filters.is_empty() {
        let mut where_conditions: Vec<String> = Vec::new();
        
        for (field, value) in &query_params.filters {
            // Skip ordering parameters
            if field == "order_by" || field == "direction" {
                continue;
            }
            
            // Validate field name to prevent SQL injection
            if field.chars().all(|c| c.is_alphanumeric() || c == '_') {
                where_conditions.push(format!("{} = ${}", field, param_index));
                sql_params.push(value.clone());
                param_index += 1;
            } else {
                return Err((StatusCode::BAD_REQUEST, format!("Invalid field name: {}", field)));
            }
        }
        
        if !where_conditions.is_empty() {
            query.push_str(&(" WHERE ".to_owned() + &where_conditions.join(" AND ")));
        }
    }
    
    // Validate and apply ordering if provided
    if let Some(order_by) = &query_params.order_by {
        // Validate order_by column name to prevent SQL injection
        // Only allow alphanumeric characters and underscores
        if order_by.chars().all(|c| c.is_alphanumeric() || c == '_') {
            // Validate direction parameter
            let direction = match &query_params.direction {
                Some(dir) if dir.to_lowercase() == "desc" => "DESC",
                _ => "ASC",
            };
            
            query.push_str(&format!(" ORDER BY {} {}", *order_by, direction));
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid order_by parameter".to_string()));
        }
    }

    // Execute query with parameters
    let mut query_builder = sqlx::query_as::<_, ConversationParticipants>(&query);
    for param in &sql_params {
        query_builder = query_builder.bind(param);
    }

    let elemints: Vec<ConversationParticipants> = query_builder.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"participant_id": elemint.participant_id, 
	"conversation_id": elemint.conversation_id, 
	"user_id": elemint.user_id, 
	"joined_at": elemint.joined_at
        })
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}

#[derive(Debug, Deserialize)]
struct conversation_participantsparticipant_idQuery {
    participant_id: uuid::Uuid,
}

pub async fn get_one_conversation_participantsparticipant_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsparticipant_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversation_participantsparticipant_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversation_participantsparticipant_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsparticipant_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversation_participants WHERE participant_id = $1");
    let q = sqlx::query_as::<_, ConversationParticipants>(&query).bind(match_val.participant_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"participant_id": elemint.participant_id, 
	"conversation_id": elemint.conversation_id, 
	"user_id": elemint.user_id, 
	"joined_at": elemint.joined_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with participant_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct conversation_participantsconversation_idQuery {
    conversation_id: uuid::Uuid,
}

pub async fn get_one_conversation_participantsconversation_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsconversation_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversation_participantsconversation_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversation_participantsconversation_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsconversation_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversation_participants WHERE conversation_id = $1");
    let q = sqlx::query_as::<_, ConversationParticipants>(&query).bind(match_val.conversation_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"participant_id": elemint.participant_id, 
	"conversation_id": elemint.conversation_id, 
	"user_id": elemint.user_id, 
	"joined_at": elemint.joined_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with conversation_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct conversation_participantsuser_idQuery {
    user_id: uuid::Uuid,
}

pub async fn get_one_conversation_participantsuser_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsuser_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversation_participantsuser_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversation_participantsuser_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsuser_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversation_participants WHERE user_id = $1");
    let q = sqlx::query_as::<_, ConversationParticipants>(&query).bind(match_val.user_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"participant_id": elemint.participant_id, 
	"conversation_id": elemint.conversation_id, 
	"user_id": elemint.user_id, 
	"joined_at": elemint.joined_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with user_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct conversation_participantsjoined_atQuery {
    joined_at: chrono::DateTime<Utc>,
}

pub async fn get_one_conversation_participantsjoined_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsjoined_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_conversation_participantsjoined_at(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_conversation_participantsjoined_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<conversation_participantsjoined_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM conversation_participants WHERE joined_at = $1");
    let q = sqlx::query_as::<_, ConversationParticipants>(&query).bind(match_val.joined_at.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"participant_id": elemint.participant_id, 
	"conversation_id": elemint.conversation_id, 
	"user_id": elemint.user_id, 
	"joined_at": elemint.joined_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with joined_at = the value"))),
    }
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Messages {
    message_id: Option<uuid::Uuid>,
    conversation_id: uuid::Uuid,
    sender_id: uuid::Uuid,
    content: String,
    sent_at: Option<chrono::DateTime<Utc>>,
    edited_at: chrono::DateTime<Utc>,
}

pub async fn add_messages(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Messages>,
) -> Json<Value> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_add_messages(extract::State(pool), Json(payload)).await;
    result
}

pub async fn data_add_messages(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Messages>,
) -> Json<Value> {
    let query = "INSERT INTO messages (conversation_id, sender_id, content, edited_at) VALUES ($1, $2, $3, $4) RETURNING *";
    
    let q = sqlx::query_as::<_, Messages>(&query)
		.bind(payload.conversation_id)
		.bind(payload.sender_id)
		.bind(payload.content)
		.bind(payload.edited_at);
    
    let result = q.fetch_one(&pool).await;

    match result {
        Ok(value) => Json(json!({"res": "success", "data": value})),
        Err(e) => Json(json!({"res": format!("error: {}", e)}))
    }
}


#[derive(Deserialize)]
struct messagesQueryParams {
    order_by: Option<String>,
    direction: Option<String>, // "asc" or "desc"
    #[serde(flatten)]
    filters: HashMap<String, String>,
}


pub async fn get_messages(
    extract::State(pool): extract::State<PgPool>,
    Query(query_params): Query<messagesQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_messages(extract::State(pool), axum::extract::Query(query_params)).await;
    result
}



pub async fn data_get_messages(
    extract::State(pool): extract::State<PgPool>,
    query_params: axum::extract::Query<messagesQueryParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut query = "SELECT * FROM messages".to_owned();
    let mut sql_params: Vec<String> = Vec::new();
    let mut param_index = 1;
    
    // Handle filters
    if !query_params.filters.is_empty() {
        let mut where_conditions: Vec<String> = Vec::new();
        
        for (field, value) in &query_params.filters {
            // Skip ordering parameters
            if field == "order_by" || field == "direction" {
                continue;
            }
            
            // Validate field name to prevent SQL injection
            if field.chars().all(|c| c.is_alphanumeric() || c == '_') {
                where_conditions.push(format!("{} = ${}", field, param_index));
                sql_params.push(value.clone());
                param_index += 1;
            } else {
                return Err((StatusCode::BAD_REQUEST, format!("Invalid field name: {}", field)));
            }
        }
        
        if !where_conditions.is_empty() {
            query.push_str(&(" WHERE ".to_owned() + &where_conditions.join(" AND ")));
        }
    }
    
    // Validate and apply ordering if provided
    if let Some(order_by) = &query_params.order_by {
        // Validate order_by column name to prevent SQL injection
        // Only allow alphanumeric characters and underscores
        if order_by.chars().all(|c| c.is_alphanumeric() || c == '_') {
            // Validate direction parameter
            let direction = match &query_params.direction {
                Some(dir) if dir.to_lowercase() == "desc" => "DESC",
                _ => "ASC",
            };
            
            query.push_str(&format!(" ORDER BY {} {}", *order_by, direction));
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid order_by parameter".to_string()));
        }
    }

    // Execute query with parameters
    let mut query_builder = sqlx::query_as::<_, Messages>(&query);
    for param in &sql_params {
        query_builder = query_builder.bind(param);
    }

    let elemints: Vec<Messages> = query_builder.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"message_id": elemint.message_id, 
	"conversation_id": elemint.conversation_id, 
	"sender_id": elemint.sender_id, 
	"content": elemint.content, 
	"sent_at": elemint.sent_at, 
	"edited_at": elemint.edited_at
        })
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}

#[derive(Debug, Deserialize)]
struct messagesmessage_idQuery {
    message_id: uuid::Uuid,
}

pub async fn get_one_messagesmessage_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagesmessage_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_messagesmessage_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_messagesmessage_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagesmessage_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM messages WHERE message_id = $1");
    let q = sqlx::query_as::<_, Messages>(&query).bind(match_val.message_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"message_id": elemint.message_id, 
	"conversation_id": elemint.conversation_id, 
	"sender_id": elemint.sender_id, 
	"content": elemint.content, 
	"sent_at": elemint.sent_at, 
	"edited_at": elemint.edited_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with message_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct messagesconversation_idQuery {
    conversation_id: uuid::Uuid,
}

pub async fn get_one_messagesconversation_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagesconversation_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_messagesconversation_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_messagesconversation_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagesconversation_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM messages WHERE conversation_id = $1");
    let q = sqlx::query_as::<_, Messages>(&query).bind(match_val.conversation_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"message_id": elemint.message_id, 
	"conversation_id": elemint.conversation_id, 
	"sender_id": elemint.sender_id, 
	"content": elemint.content, 
	"sent_at": elemint.sent_at, 
	"edited_at": elemint.edited_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with conversation_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct messagessender_idQuery {
    sender_id: uuid::Uuid,
}

pub async fn get_one_messagessender_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagessender_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_messagessender_id(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_messagessender_id(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagessender_idQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM messages WHERE sender_id = $1");
    let q = sqlx::query_as::<_, Messages>(&query).bind(match_val.sender_id.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"message_id": elemint.message_id, 
	"conversation_id": elemint.conversation_id, 
	"sender_id": elemint.sender_id, 
	"content": elemint.content, 
	"sent_at": elemint.sent_at, 
	"edited_at": elemint.edited_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with sender_id = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct messagescontentQuery {
    content: String,
}

pub async fn get_one_messagescontent(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagescontentQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_messagescontent(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_messagescontent(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagescontentQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM messages WHERE content = $1");
    let q = sqlx::query_as::<_, Messages>(&query).bind(match_val.content.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"message_id": elemint.message_id, 
	"conversation_id": elemint.conversation_id, 
	"sender_id": elemint.sender_id, 
	"content": elemint.content, 
	"sent_at": elemint.sent_at, 
	"edited_at": elemint.edited_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with content = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct messagessent_atQuery {
    sent_at: chrono::DateTime<Utc>,
}

pub async fn get_one_messagessent_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagessent_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_messagessent_at(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_messagessent_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagessent_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM messages WHERE sent_at = $1");
    let q = sqlx::query_as::<_, Messages>(&query).bind(match_val.sent_at.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"message_id": elemint.message_id, 
	"conversation_id": elemint.conversation_id, 
	"sender_id": elemint.sender_id, 
	"content": elemint.content, 
	"sent_at": elemint.sent_at, 
	"edited_at": elemint.edited_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with sent_at = the value"))),
    }
}



#[derive(Debug, Deserialize)]
struct messagesedited_atQuery {
    edited_at: chrono::DateTime<Utc>,
}

pub async fn get_one_messagesedited_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagesedited_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Call data function from data module 
    // Other business logic can also be handled here 
    let result = data_get_one_messagesedited_at(extract::State(pool), match_val).await;
    result
}

pub async fn data_get_one_messagesedited_at(
    extract::State(pool): extract::State<PgPool>,
    match_val: Query<messagesedited_atQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = format!("SELECT * FROM messages WHERE edited_at = $1");
    let q = sqlx::query_as::<_, Messages>(&query).bind(match_val.edited_at.clone());

    let elemint = q.fetch_optional(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database err{}", e))
    })?;

    match elemint {
        Some(elemint) => Ok(Json(json!({
            "payload": {
                	"message_id": elemint.message_id, 
	"conversation_id": elemint.conversation_id, 
	"sender_id": elemint.sender_id, 
	"content": elemint.content, 
	"sent_at": elemint.sent_at, 
	"edited_at": elemint.edited_at, 

            }
        }))),
        None => Err((StatusCode::NOT_FOUND, format!("No record found with edited_at = the value"))),
    }
}





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


async fn health() -> String {"healthy".to_string() }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")
     .unwrap_or_else(|_| "postgres://dbuser:p@localhost:1111/data".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let migrate = sqlx::migrate!("./migrations").run(&pool).await;

    match migrate {
        Ok(_) => println!("Migrations applied successfully."),
        Err(e) => eprintln!("Error applying migrations: {}", e),
    };

    let static_service =
        ServeDir::new("frontend/build").not_found_service(service_fn(|_req| async {
            match tokio::fs::read_to_string("frontend/build/index.html").await {
                Ok(body) => Ok((StatusCode::OK, Html(body)).into_response()),
                Err(err) => Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read index.html: {}", err),
                )
                    .into_response()),
            }
        }));


    let app = Router::new()
    .route("/health", get(health))
    	.route("/add_users", post(add_users))
	.route("/get_users", get(get_users))
	.route("/get_one_usersuser_id", get(get_one_usersuser_id))
	.route("/get_one_usersusername", get(get_one_usersusername))
	.route("/get_one_usersemail", get(get_one_usersemail))
	.route("/get_one_usersdisplay_name", get(get_one_usersdisplay_name))
	.route("/get_one_userscreated_at", get(get_one_userscreated_at))
	.route("/add_conversations", post(add_conversations))
	.route("/get_conversations", get(get_conversations))
	.route("/get_one_conversationsconversation_id", get(get_one_conversationsconversation_id))
	.route("/get_one_conversationsname", get(get_one_conversationsname))
	.route("/get_one_conversationsis_group_chat", get(get_one_conversationsis_group_chat))
	.route("/get_one_conversationscreated_at", get(get_one_conversationscreated_at))
	.route("/add_conversation_participants", post(add_conversation_participants))
	.route("/get_conversation_participants", get(get_conversation_participants))
	.route("/get_one_conversation_participantsparticipant_id", get(get_one_conversation_participantsparticipant_id))
	.route("/get_one_conversation_participantsconversation_id", get(get_one_conversation_participantsconversation_id))
	.route("/get_one_conversation_participantsuser_id", get(get_one_conversation_participantsuser_id))
	.route("/get_one_conversation_participantsjoined_at", get(get_one_conversation_participantsjoined_at))
	.route("/add_messages", post(add_messages))
	.route("/get_messages", get(get_messages))
	.route("/get_one_messagesmessage_id", get(get_one_messagesmessage_id))
	.route("/get_one_messagesconversation_id", get(get_one_messagesconversation_id))
	.route("/get_one_messagessender_id", get(get_one_messagessender_id))
	.route("/get_one_messagescontent", get(get_one_messagescontent))
	.route("/get_one_messagessent_at", get(get_one_messagessent_at))
	.route("/get_one_messagesedited_at", get(get_one_messagesedited_at))
	.route("/signed-urls/:video_path", get(get_signed_url))

    .route("/python", get(python))
    .fallback_service(static_service)
    .layer(
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(vec![
                "http://localhost:3000".parse().unwrap(),
                "https://example.com".parse().unwrap(),
            ]))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(tower_http::cors::Any)
    )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}



async fn generate_signed_url(object_key: String) -> Result<String, anyhow::Error> {
    let endpoint = env::var("MINIO_ENDPOINT")
        .unwrap_or_else(|_| "localhost:9001".to_string());
    let access_key = env::var("MINIO_ACCESS_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let secret_key = env::var("MINIO_SECRET_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let bucket = env::var("MINIO_BUCKET").unwrap_or_else(|_| "bucket".to_string());
    let endpoint = env::var("MINIO_ENDPOINT").unwrap_or_else(|_| "localhost:9000".to_string());
    let secure = env::var("MINIO_SECURE")
        .map(|s| s.to_lowercase() == "true")
        .unwrap_or(false);

    let provider = StaticProvider::new(&access_key, &secret_key, None);

    let minio = Minio::builder()
        .endpoint(&endpoint)
        .provider(provider)
        .secure(secure)
        .region("us-east-1".to_string())  // Explicitly set region to match MinIO default
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create MinIO client: {}", e))?;

    let presigned_url = minio
        .presigned_get_object(
            PresignedArgs::new(bucket, object_key)
                .expires(3600),  // 1 hour in seconds
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to generate presigned URL: {}", e))?;
    Ok(presigned_url)
}
    

async fn get_signed_url(
    Path(video_path): Path<String>,
) -> impl IntoResponse {
    let object_key = video_path; 
    println!("Environment variables:");
    println!("MINIO_ENDPOINT: {}", env::var("MINIO_ENDPOINT").unwrap_or_else(|_| "not set".to_string()));
    println!("MINIO_BUCKET: {}", env::var("MINIO_BUCKET").unwrap_or_else(|_| "not set, using default 'test'".to_string()));
    
    match generate_signed_url(object_key).await {
        Ok(url) => (StatusCode::OK, url).into_response(),
        Err(e) => {
            eprintln!("Error generating signed URL: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to generate signed URL: {}", e)).into_response()
        }
    }
}
async fn upload_video(
    // mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, String)> {
    let provider = StaticProvider::new("minioadmin", "minioadmin", None);
    let minio = Minio::builder()
        .endpoint("minio:9000")
        .provider(provider)
        .secure(false)
        .build()
        .unwrap();

        let _data = "hello minio";

        let upload_result = minio.put_object("bucket", "file.txt", _data.into()).await;
        
        return Ok(Json(json!({
            "status": upload_result.is_ok(),
            "message": if upload_result.is_ok() {
                "File uploaded successfully"
            } else {
                "Failed to upload file"
            },
            "file_name": "file.txt"
        })));
}
    