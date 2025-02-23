use anyhow::anyhow;
use once_cell::sync::Lazy;
use redis::{aio::ConnectionManager, AsyncCommands, Client};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

// Global Redis connection manager
pub static GLOBAL_REDIS_CLIENT: Lazy<Arc<Mutex<Option<ConnectionManager>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

// Initialize Redis connection once
pub async fn init_redis(redis_url: &str) -> anyhow::Result<()> {
    let client = Client::open(redis_url).expect("Failed to connect to Redis");
    let conn_manager = ConnectionManager::new(client)
        .await
        .expect("Failed to create connection manager");
    let mut global_conn = GLOBAL_REDIS_CLIENT.lock().await;
    *global_conn = Some(conn_manager);
    Ok(())
}

// Get a connection from the global Redis manager
pub async fn get_connection() -> Result<ConnectionManager, redis::RedisError> {
    let global_conn = GLOBAL_REDIS_CLIENT.lock().await;
    if let Some(conn) = &*global_conn {
        Ok(conn.clone())
    } else {
        Err(redis::RedisError::from((
            redis::ErrorKind::IoError,
            "Redis not initialized",
        )))
    }
}

/* -------------------- STRING OPERATIONS -------------------- */
pub async fn set_value(key: &str, value: &str) -> anyhow::Result<()> {
    let mut conn = get_connection().await?;
    conn.set(key, value).await?;
    Ok(())
}

pub async fn get_value(key: &str) -> anyhow::Result<Option<String>> {
    let mut conn = get_connection().await?;
    let result: Option<String> = conn.get(key).await?;
    Ok(result)
}

/* -------------------- HASH OPERATIONS -------------------- */
// Set a field in a Redis hash
pub async fn set_hash<T>(key: &str, field: &str, value: T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let mut conn = get_connection().await?;
    let json = serde_json::to_string(&value)?;
    conn.hset(key, field, json).await?;
    Ok(())
}

pub async fn set_hash_object(key: &str, field: &str, value: &str) -> anyhow::Result<()> {
    let mut conn = get_connection().await?;
    conn.hset(key, field, value).await?;
    Ok(())
}

pub async fn get_hash(key: &str, field: &str) -> anyhow::Result<Option<String>> {
    let mut conn = get_connection().await?;
    let result: Option<String> = conn.hget(key, field).await?;
    Ok(result)
}
pub async fn get_hash_object<T>(key: &str, field: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned + Sync,
{
    let mut conn = get_connection().await?;
    let result: Option<String> = conn.hget(key, field).await?;

    if let Some(value) = result {
        let data: T = serde_json::from_str(&value)?;
        return Ok(data);
    }

    Err(anyhow!("Hash key not found"))
}

pub async fn delete_hash(key: &str, field: &str) -> anyhow::Result<()> {
    let mut conn = get_connection().await?;
    conn.hdel(key, field).await?;
    Ok(())
}
