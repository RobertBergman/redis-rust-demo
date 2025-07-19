mod common;

use redis_rust_demo::{RedisClient, Result};
use redis_rust_demo::demos::{BasicOpsDemo, ListDemo, SetDemo, HashDemo};
use redis_rust_demo::models::User;
use redis::AsyncCommands;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_full_user_workflow() -> Result<()> {
    let client = RedisClient::new("redis://localhost:6379/14")?;
    let mut conn = client.get_async_connection().await?;
    
    // Create a user
    let user = User::new(
        "integration_user".to_string(),
        "integration@test.com".to_string(),
        "Integration Test User".to_string()
    );
    
    // Store user as JSON
    let user_json = serde_json::to_string(&user)?;
    conn.set(&user.redis_key(), &user_json).await?;
    
    // Create indexes
    conn.set(&user.username_index_key(), &user.id.to_string()).await?;
    conn.set(&user.email_index_key(), &user.id.to_string()).await?;
    
    // Retrieve user by username index
    let user_id: String = conn.get(&user.username_index_key()).await?;
    assert_eq!(user_id, user.id.to_string());
    
    // Retrieve user data
    let stored_json: String = conn.get(&user.redis_key()).await?;
    let retrieved_user: User = serde_json::from_str(&stored_json)?;
    
    assert_eq!(retrieved_user.username, user.username);
    assert_eq!(retrieved_user.email, user.email);
    
    // Update user login count
    conn.hincr(&user.redis_key(), "login_count", 1).await?;
    
    // Clean up
    conn.del(vec![
        user.redis_key(),
        user.username_index_key(),
        user.email_index_key()
    ]).await?;
    
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_all_demos_run_successfully() -> Result<()> {
    let client = RedisClient::new("redis://localhost:6379/14")?;
    
    // Test string operations
    let basic_demo = BasicOpsDemo::new(client.clone());
    basic_demo.string_operations().await?;
    basic_demo.key_operations().await?;
    
    // Test list operations
    let list_demo = ListDemo::new(client.clone());
    list_demo.demonstrate().await?;
    
    // Test set operations
    let set_demo = SetDemo::new(client.clone());
    set_demo.demonstrate().await?;
    
    // Test hash operations
    let hash_demo = HashDemo::new(client.clone());
    hash_demo.demonstrate().await?;
    
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_concurrent_operations() -> Result<()> {
    let client = RedisClient::new("redis://localhost:6379/14")?;
    let mut conn = client.get_async_connection().await?;
    
    // Set up a counter
    conn.set("concurrent_counter", 0).await?;
    
    // Run concurrent increments
    let mut handles = vec![];
    
    for i in 0..10 {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let mut conn = client_clone.get_async_connection().await.unwrap();
            for _ in 0..100 {
                let _: i64 = conn.incr("concurrent_counter", 1).await.unwrap();
            }
            i
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Check final value
    let final_value: i64 = conn.get("concurrent_counter").await?;
    assert_eq!(final_value, 1000);
    
    // Clean up
    conn.del("concurrent_counter").await?;
    
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_transaction_workflow() -> Result<()> {
    let client = RedisClient::new("redis://localhost:6379/14")?;
    let mut conn = client.get_async_connection().await?;
    
    // Start a transaction
    let mut pipe = redis::pipe();
    pipe.atomic()
        .set("tx_key1", "value1")
        .set("tx_key2", "value2")
        .incr("tx_counter", 1)
        .incr("tx_counter", 1);
    
    let _results: Vec<String> = pipe.query_async(&mut conn).await?;
    
    // Verify transaction results
    let value1: String = conn.get("tx_key1").await?;
    let value2: String = conn.get("tx_key2").await?;
    let counter: i64 = conn.get("tx_counter").await?;
    
    assert_eq!(value1, "value1");
    assert_eq!(value2, "value2");
    assert_eq!(counter, 2);
    
    // Clean up
    conn.del(vec!["tx_key1", "tx_key2", "tx_counter"]).await?;
    
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_error_handling() {
    let client = RedisClient::new("redis://localhost:6379/14").unwrap();
    let mut conn = client.get_async_connection().await.unwrap();
    
    // Test getting non-existent key
    let result: redis::RedisResult<Option<String>> = conn.get("non_existent_key").await;
    assert!(result.is_ok() && result.unwrap().is_none());
    
    // Test invalid operations
    let _: () = conn.set("string_key", "string_value").await.unwrap();
    let result: redis::RedisResult<Vec<String>> = conn.lrange("string_key", 0, -1).await;
    assert!(result.is_err());
    
    // Clean up
    let _: () = conn.del("string_key").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_pattern_matching() -> Result<()> {
    let client = RedisClient::new("redis://localhost:6379/14")?;
    let mut conn = client.get_async_connection().await?;
    
    // Create keys with pattern
    for i in 0..5 {
        conn.set(format!("pattern:test:{}", i), i).await?;
    }
    
    // Find keys matching pattern
    let keys: Vec<String> = redis::cmd("KEYS")
        .arg("pattern:test:*")
        .query_async(&mut conn)
        .await?;
    
    assert_eq!(keys.len(), 5);
    
    // Clean up
    if !keys.is_empty() {
        conn.del(keys).await?;
    }
    
    Ok(())
}