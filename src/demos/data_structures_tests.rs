#[cfg(test)]
mod list_tests {
    use crate::demos::ListDemo;
    use crate::RedisClient;
    use redis::AsyncCommands;
    
    async fn get_test_client() -> RedisClient {
        RedisClient::new("redis://localhost:6379/15").unwrap()
    }
    
    async fn cleanup_keys(client: &RedisClient, keys: &[&str]) {
        let mut conn = client.get_async_connection().await.unwrap();
        // First do a FLUSHDB to ensure clean state
        let _: String = redis::cmd("FLUSHDB")
            .query_async(&mut conn)
            .await
            .unwrap_or_default();
    }
    
    #[tokio::test]
    async fn test_list_push_pop() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["test_list"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test LPUSH and RPUSH
        let _: () = conn.lpush("test_list", vec!["a", "b"]).await.unwrap();
        let _: () = conn.rpush("test_list", vec!["c", "d"]).await.unwrap();
        
        let list: Vec<String> = conn.lrange("test_list", 0, -1).await.unwrap();
        assert_eq!(list, vec!["b", "a", "c", "d"]);
        
        // Test LPOP and RPOP
        let left: Option<String> = conn.lpop("test_list", None).await.unwrap();
        let right: Option<String> = conn.rpop("test_list", None).await.unwrap();
        
        assert_eq!(left, Some("b".to_string()));
        assert_eq!(right, Some("d".to_string()));
        
        cleanup_keys(&client, &["test_list"]).await;
    }
    
    #[tokio::test]
    async fn test_list_operations() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["test_list"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Create list
        let _: () = conn.rpush("test_list", vec!["one", "two", "three"]).await.unwrap();
        
        // Test LLEN
        let len: usize = conn.llen("test_list").await.unwrap();
        assert_eq!(len, 3);
        
        // Test LINDEX
        let elem: Option<String> = conn.lindex("test_list", 1).await.unwrap();
        assert_eq!(elem, Some("two".to_string()));
        
        // Test LINSERT
        let _: () = conn.linsert_before("test_list", "two", "one-half").await.unwrap();
        let list: Vec<String> = conn.lrange("test_list", 0, -1).await.unwrap();
        assert_eq!(list, vec!["one", "one-half", "two", "three"]);
        
        cleanup_keys(&client, &["test_list"]).await;
    }
    
    #[tokio::test]
    async fn test_list_demo_full() {
        let client = get_test_client().await;
        let demo = ListDemo::new(client.clone());
        
        let result = demo.demonstrate().await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod set_tests {
    use crate::demos::SetDemo;
    use crate::RedisClient;
    use redis::AsyncCommands;
    
    async fn get_test_client() -> RedisClient {
        RedisClient::new("redis://localhost:6379/15").unwrap()
    }
    
    async fn cleanup_keys(client: &RedisClient, keys: &[&str]) {
        let mut conn = client.get_async_connection().await.unwrap();
        // First do a FLUSHDB to ensure clean state
        let _: String = redis::cmd("FLUSHDB")
            .query_async(&mut conn)
            .await
            .unwrap_or_default();
    }
    
    #[tokio::test]
    async fn test_set_add_members() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["test_set"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test SADD
        let _: () = conn.sadd("test_set", vec!["a", "b", "c"]).await.unwrap();
        let _: () = conn.sadd("test_set", "a").await.unwrap(); // Duplicate
        
        // Test SCARD
        let count: usize = conn.scard("test_set").await.unwrap();
        assert_eq!(count, 3);
        
        // Test SMEMBERS
        let members: Vec<String> = conn.smembers("test_set").await.unwrap();
        assert_eq!(members.len(), 3);
        assert!(members.contains(&"a".to_string()));
        assert!(members.contains(&"b".to_string()));
        assert!(members.contains(&"c".to_string()));
        
        cleanup_keys(&client, &["test_set"]).await;
    }
    
    #[tokio::test]
    async fn test_set_operations() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["set1", "set2", "set3"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Create sets
        let _: () = conn.sadd("set1", vec!["a", "b", "c"]).await.unwrap();
        let _: () = conn.sadd("set2", vec!["b", "c", "d"]).await.unwrap();
        
        // Test SINTER
        let inter: Vec<String> = conn.sinter(&["set1", "set2"]).await.unwrap();
        assert_eq!(inter.len(), 2);
        assert!(inter.contains(&"b".to_string()));
        assert!(inter.contains(&"c".to_string()));
        
        // Test SUNION
        let union: Vec<String> = conn.sunion(&["set1", "set2"]).await.unwrap();
        assert_eq!(union.len(), 4);
        
        // Test SDIFF
        let diff: Vec<String> = conn.sdiff(&["set1", "set2"]).await.unwrap();
        assert_eq!(diff, vec!["a"]);
        
        cleanup_keys(&client, &["set1", "set2", "set3"]).await;
    }
    
    #[tokio::test]
    async fn test_set_membership() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["test_set"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        let _: () = conn.sadd("test_set", vec!["apple", "banana"]).await.unwrap();
        
        // Test SISMEMBER
        let is_member: bool = conn.sismember("test_set", "apple").await.unwrap();
        let not_member: bool = conn.sismember("test_set", "orange").await.unwrap();
        
        assert!(is_member);
        assert!(!not_member);
        
        // Test SREM
        let _: () = conn.srem("test_set", "apple").await.unwrap();
        let is_member_after: bool = conn.sismember("test_set", "apple").await.unwrap();
        assert!(!is_member_after);
        
        cleanup_keys(&client, &["test_set"]).await;
    }
    
    #[tokio::test]
    async fn test_set_demo_full() {
        let client = get_test_client().await;
        let demo = SetDemo::new(client.clone());
        
        let result = demo.demonstrate().await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod hash_tests {
    use crate::demos::HashDemo;
    use crate::RedisClient;
    use redis::AsyncCommands;
    use std::collections::HashMap;
    
    async fn get_test_client() -> RedisClient {
        RedisClient::new("redis://localhost:6379/15").unwrap()
    }
    
    async fn cleanup_keys(client: &RedisClient, keys: &[&str]) {
        let mut conn = client.get_async_connection().await.unwrap();
        // First do a FLUSHDB to ensure clean state
        let _: String = redis::cmd("FLUSHDB")
            .query_async(&mut conn)
            .await
            .unwrap_or_default();
    }
    
    #[tokio::test]
    async fn test_hash_set_get() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["test_hash"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test HSET and HGET
        let _: () = conn.hset("test_hash", "field1", "value1").await.unwrap();
        let value: String = conn.hget("test_hash", "field1").await.unwrap();
        assert_eq!(value, "value1");
        
        // Test HSET multiple
        let _: () = conn.hset_multiple("test_hash", &[
            ("field2", "value2"),
            ("field3", "value3"),
        ]).await.unwrap();
        
        // Test HGETALL
        let hash: HashMap<String, String> = conn.hgetall("test_hash").await.unwrap();
        assert_eq!(hash.len(), 3);
        assert_eq!(hash.get("field1"), Some(&"value1".to_string()));
        assert_eq!(hash.get("field2"), Some(&"value2".to_string()));
        assert_eq!(hash.get("field3"), Some(&"value3".to_string()));
        
        cleanup_keys(&client, &["test_hash"]).await;
    }
    
    #[tokio::test]
    async fn test_hash_operations() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["test_hash"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Set up hash
        let _: () = conn.hset_multiple("test_hash", &[
            ("name", "John"),
            ("age", "30"),
            ("city", "NYC"),
        ]).await.unwrap();
        
        // Test HKEYS
        let keys: Vec<String> = conn.hkeys("test_hash").await.unwrap();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"name".to_string()));
        
        // Test HVALS
        let vals: Vec<String> = conn.hvals("test_hash").await.unwrap();
        assert_eq!(vals.len(), 3);
        assert!(vals.contains(&"John".to_string()));
        
        // Test HEXISTS
        let exists: bool = conn.hexists("test_hash", "name").await.unwrap();
        let not_exists: bool = conn.hexists("test_hash", "phone").await.unwrap();
        assert!(exists);
        assert!(!not_exists);
        
        // Test HDEL
        let _: () = conn.hdel("test_hash", "city").await.unwrap();
        let exists_after: bool = conn.hexists("test_hash", "city").await.unwrap();
        assert!(!exists_after);
        
        cleanup_keys(&client, &["test_hash"]).await;
    }
    
    #[tokio::test]
    async fn test_hash_increment() {
        let client = get_test_client().await;
        cleanup_keys(&client, &["test_hash"]).await;
        
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test HINCRBY
        let _: () = conn.hincr("test_hash", "counter", 5).await.unwrap();
        let _: () = conn.hincr("test_hash", "counter", 3).await.unwrap();
        
        let value: i64 = conn.hget("test_hash", "counter").await.unwrap();
        assert_eq!(value, 8);
        
        cleanup_keys(&client, &["test_hash"]).await;
    }
    
    #[tokio::test]
    async fn test_hash_demo_full() {
        let client = get_test_client().await;
        let demo = HashDemo::new(client.clone());
        
        let result = demo.demonstrate().await;
        assert!(result.is_ok());
    }
}