use crate::{RedisClient, Result};
use redis::AsyncCommands;
use tracing::info;

pub struct BasicOpsDemo {
    client: RedisClient,
}

impl BasicOpsDemo {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    pub async fn string_operations(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== String Operations Demo ===\n");
        
        // SET and GET
        println!("1. SET and GET:");
        conn.set("message", "Hello, Redis!").await?;
        let value: String = conn.get("message").await?;
        println!("   SET message 'Hello, Redis!'");
        println!("   GET message => '{}'", value);
        
        // SET with expiration
        println!("\n2. SET with expiration (EX):");
        conn.set_ex("temp_key", "This will expire", 5).await?;
        let ttl: i64 = conn.ttl("temp_key").await?;
        println!("   SET temp_key 'This will expire' EX 5");
        println!("   TTL temp_key => {} seconds", ttl);
        
        // INCR and DECR
        println!("\n3. INCR and DECR:");
        conn.set("counter", 10).await?;
        let incr_result: i64 = conn.incr("counter", 1).await?;
        println!("   SET counter 10");
        println!("   INCR counter => {}", incr_result);
        
        let decr_result: i64 = conn.decr("counter", 3).await?;
        println!("   DECRBY counter 3 => {}", decr_result);
        
        // MSET and MGET
        println!("\n4. MSET and MGET (multiple keys):");
        conn.mset(&[
            ("key1", "value1"),
            ("key2", "value2"),
            ("key3", "value3"),
        ]).await?;
        
        let values: Vec<Option<String>> = conn.get::<_, Vec<Option<String>>>(
            &["key1", "key2", "key3", "key4"]
        ).await?;
        
        println!("   MSET key1 value1 key2 value2 key3 value3");
        println!("   MGET key1 key2 key3 key4 => {:?}", values);
        
        // APPEND
        println!("\n5. APPEND:");
        conn.set("greeting", "Hello").await?;
        let len: usize = conn.append("greeting", ", World!").await?;
        let final_value: String = conn.get("greeting").await?;
        println!("   SET greeting 'Hello'");
        println!("   APPEND greeting ', World!' => length: {}", len);
        println!("   GET greeting => '{}'", final_value);
        
        // STRLEN
        println!("\n6. STRLEN:");
        let str_len: usize = conn.strlen("greeting").await?;
        println!("   STRLEN greeting => {}", str_len);
        
        // GETRANGE
        println!("\n7. GETRANGE (substring):");
        let substring: String = conn.getrange("greeting", 0, 4).await?;
        println!("   GETRANGE greeting 0 4 => '{}'", substring);
        
        // EXISTS and DEL
        println!("\n8. EXISTS and DEL:");
        let exists: bool = conn.exists("message").await?;
        println!("   EXISTS message => {}", exists);
        
        let deleted: usize = conn.del(&["message", "counter", "greeting"]).await?;
        println!("   DEL message counter greeting => {} keys deleted", deleted);
        
        info!("String operations demo completed");
        Ok(())
    }

    pub async fn key_operations(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Key Management Demo ===\n");
        
        // Create some test keys
        conn.set("user:1000:name", "Alice").await?;
        conn.set("user:1000:email", "alice@example.com").await?;
        conn.set("user:1001:name", "Bob").await?;
        conn.set("session:abc123", "active").await?;
        conn.set_ex("temp:data", "temporary", 10).await?;
        
        // KEYS pattern (not recommended for production)
        println!("1. KEYS pattern:");
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg("user:*")
            .query_async(&mut conn)
            .await?;
        println!("   KEYS user:* => {:?}", keys);
        
        // SCAN (recommended for production)
        println!("\n2. SCAN (production-safe):");
        let mut scan_keys = Vec::new();
        let mut cursor = 0;
        loop {
            let (new_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg("user:*")
                .arg("COUNT")
                .arg(10)
                .query_async(&mut conn)
                .await?;
            
            scan_keys.extend(batch);
            cursor = new_cursor;
            
            if cursor == 0 {
                break;
            }
        }
        println!("   SCAN with MATCH user:* => Found {} keys", scan_keys.len());
        
        // TYPE
        println!("\n3. TYPE:");
        let key_type: String = redis::cmd("TYPE")
            .arg("user:1000:name")
            .query_async(&mut conn)
            .await?;
        println!("   TYPE user:1000:name => {}", key_type);
        
        // EXPIRE and TTL
        println!("\n4. EXPIRE and TTL:");
        conn.expire("session:abc123", 60).await?;
        let ttl: i64 = conn.ttl("session:abc123").await?;
        println!("   EXPIRE session:abc123 60");
        println!("   TTL session:abc123 => {} seconds", ttl);
        
        // PERSIST
        println!("\n5. PERSIST (remove expiration):");
        conn.persist("session:abc123").await?;
        let ttl_after: i64 = conn.ttl("session:abc123").await?;
        println!("   PERSIST session:abc123");
        println!("   TTL session:abc123 => {} (-1 means no expiration)", ttl_after);
        
        // RENAME
        println!("\n6. RENAME:");
        conn.rename("user:1001:name", "user:1001:fullname").await?;
        let renamed_value: String = conn.get("user:1001:fullname").await?;
        println!("   RENAME user:1001:name user:1001:fullname");
        println!("   GET user:1001:fullname => '{}'", renamed_value);
        
        // Clean up
        let pattern_keys: Vec<String> = redis::cmd("KEYS")
            .arg("*")
            .query_async(&mut conn)
            .await?;
        if !pattern_keys.is_empty() {
            conn.del(pattern_keys).await?;
        }
        
        info!("Key operations demo completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::AsyncCommands;
    
    async fn get_test_client() -> RedisClient {
        RedisClient::new("redis://localhost:6379/15").unwrap()
    }
    
    async fn cleanup_test_keys(client: &RedisClient) {
        let mut conn = client.get_async_connection().await.unwrap();
        // Use FLUSHDB to ensure clean state for tests
        let _: String = redis::cmd("FLUSHDB")
            .query_async(&mut conn)
            .await
            .unwrap_or_default();
    }
    
    #[tokio::test]
    async fn test_string_operations_set_get() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test SET and GET
        let _: () = conn.set("test_key", "test_value").await.unwrap();
        let value: String = conn.get("test_key").await.unwrap();
        assert_eq!(value, "test_value");
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_string_operations_incr_decr() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test INCR and DECR
        let _: () = conn.set("counter", 10).await.unwrap();
        let incr_result: i64 = conn.incr("counter", 5).await.unwrap();
        assert_eq!(incr_result, 15);
        
        let decr_result: i64 = conn.decr("counter", 3).await.unwrap();
        assert_eq!(decr_result, 12);
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_string_operations_mset_mget() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test MSET and MGET
        let _: () = conn.mset(&[
            ("key1", "val1"),
            ("key2", "val2"),
            ("key3", "val3"),
        ]).await.unwrap();
        
        let values: Vec<Option<String>> = conn.get(vec!["key1", "key2", "key3", "key4"]).await.unwrap();
        assert_eq!(values, vec![
            Some("val1".to_string()),
            Some("val2".to_string()),
            Some("val3".to_string()),
            None
        ]);
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_string_operations_append() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test APPEND
        let _: () = conn.set("greeting", "Hello").await.unwrap();
        let len: usize = conn.append("greeting", " World").await.unwrap();
        assert_eq!(len, 11);
        
        let value: String = conn.get("greeting").await.unwrap();
        assert_eq!(value, "Hello World");
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_string_operations_full_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = BasicOpsDemo::new(client.clone());
        let result = demo.string_operations().await;
        assert!(result.is_ok());
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_key_operations_expire_ttl() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test EXPIRE and TTL
        let _: () = conn.set("temp_key", "value").await.unwrap();
        let _: () = conn.expire("temp_key", 60).await.unwrap();
        let ttl: i64 = conn.ttl("temp_key").await.unwrap();
        assert!(ttl > 0 && ttl <= 60);
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_key_operations_exists_del() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test EXISTS and DEL
        let _: () = conn.set("test_key", "value").await.unwrap();
        let exists: bool = conn.exists("test_key").await.unwrap();
        assert!(exists);
        
        let deleted: usize = conn.del("test_key").await.unwrap();
        assert_eq!(deleted, 1);
        
        let exists_after: bool = conn.exists("test_key").await.unwrap();
        assert!(!exists_after);
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_key_operations_rename() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test RENAME
        let _: () = conn.set("old_key", "value").await.unwrap();
        let _: () = conn.rename("old_key", "new_key").await.unwrap();
        
        let old_exists: bool = conn.exists("old_key").await.unwrap();
        assert!(!old_exists);
        
        let new_value: String = conn.get("new_key").await.unwrap();
        assert_eq!(new_value, "value");
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_key_operations_full_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = BasicOpsDemo::new(client.clone());
        let result = demo.key_operations().await;
        assert!(result.is_ok());
        
        cleanup_test_keys(&client).await;
    }
}