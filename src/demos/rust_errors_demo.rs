use crate::{RedisClient, Result};
use redis::AsyncCommands;
use std::sync::Arc;

pub struct RustErrorsDemo {
    client: RedisClient,
}

impl RustErrorsDemo {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    pub async fn demonstrate_ownership_errors(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Common Rust Errors Demo: Ownership ===\n");
        
        // Example 1: Cannot move out of borrowed content
        println!("1. Cannot move out of borrowed content:");
        println!("   ❌ BAD: let first = v[0]; // Error: cannot move");
        println!("   ✅ GOOD: let first = &v[0]; // Borrow instead");
        println!("   ✅ GOOD: let first_owned = v[0].clone(); // Or clone\n");
        
        // Demonstrate with Redis
        let keys = vec!["key1", "key2", "key3"];
        for (i, key) in keys.iter().enumerate() {
            conn.set::<_, _, ()>(key, format!("value{}", i)).await?;
        }
        
        // Show proper borrowing with Redis results
        let values: Vec<Option<String>> = conn.get(&keys).await?;
        println!("   Redis Example - Processing values:");
        // Good: borrowing from the vector
        for (i, value) in values.iter().enumerate() {
            if let Some(v) = value {
                println!("   Key {}: {}", i, v);
            }
        }
        
        // Example 2: Use after move
        println!("\n2. Use after move:");
        println!("   ❌ BAD: let s2 = s; println!(\"{{}}\", s); // Error: use after move");
        println!("   ✅ GOOD: let s2 = s.clone(); // Clone if you need both");
        println!("   ✅ GOOD: let s2 = &s; // Or use references\n");
        
        // Demonstrate with owned String from Redis
        let value: String = conn.get("key1").await?;
        let value_ref = &value; // Good: borrow instead of move
        let value_clone = value.clone(); // Good: clone when you need ownership
        println!("   Original: {}, Reference: {}, Clone: {}", value, value_ref, value_clone);
        
        Ok(())
    }

    pub async fn demonstrate_lifetime_errors(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Common Rust Errors Demo: Lifetimes ===\n");
        
        println!("1. Lifetime parameter required:");
        println!("   ❌ BAD: struct Container {{ data: &str }} // Error: missing lifetime");
        println!("   ✅ GOOD: struct Container<'a> {{ data: &'a str }}");
        println!("   ✅ GOOD: struct Container {{ data: String }} // Or use owned data\n");
        
        // Demonstrate with a function that returns references
        println!("2. Function lifetime annotations:");
        println!("   ❌ BAD: fn longest(x: &str, y: &str) -> &str // Error: missing lifetime");
        println!("   ✅ GOOD: fn longest<'a>(x: &'a str, y: &'a str) -> &'a str\n");
        
        // Redis example with proper lifetime handling
        let key1 = "lifetime_test1";
        let key2 = "lifetime_test2";
        conn.set::<_, _, ()>(key1, "short").await?;
        conn.set::<_, _, ()>(key2, "much longer value").await?;
        
        let val1: String = conn.get(key1).await?;
        let val2: String = conn.get(key2).await?;
        
        // Good: return owned data instead of references
        let longest = if val1.len() > val2.len() { val1 } else { val2 };
        println!("   Longest value: {}", longest);
        
        Ok(())
    }

    pub async fn demonstrate_type_errors(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Common Rust Errors Demo: Type System ===\n");
        
        println!("1. Type annotations needed:");
        println!("   ❌ BAD: let parsed = numbers.iter().collect(); // Error: type needed");
        println!("   ✅ GOOD: let parsed: Vec<String> = numbers.iter().collect();");
        println!("   ✅ GOOD: let parsed = numbers.iter().collect::<Vec<String>>();\n");
        
        // Redis example requiring type annotations
        conn.set::<_, _, ()>("type_test", "42").await?;
        
        // Need type annotation for get
        let value: String = conn.get("type_test").await?;
        let parsed: i32 = value.parse().map_err(|e| crate::DemoError::Demo(format!("Parse error: {}", e)))?;
        println!("   Redis value as string: {}, parsed as i32: {}", value, parsed);
        
        println!("\n2. Redis-specific type annotations:");
        println!("   ❌ BAD: conn.set(\"key\", \"value\").await?; // May need type hint");
        println!("   ✅ GOOD: conn.set::<_, _, ()>(\"key\", \"value\").await?;");
        println!("   ✅ GOOD: let _: () = conn.set(\"key\", \"value\").await?;\n");
        
        // Demonstrate different ways to handle Redis return types
        let _: () = conn.set("anno_test", "value").await?;
        conn.set::<_, _, ()>("anno_test2", "value2").await?;
        
        Ok(())
    }

    pub async fn demonstrate_async_errors(&self) -> Result<()> {
        println!("\n=== Common Rust Errors Demo: Async/Await ===\n");
        
        println!("1. Cannot be sent between threads safely:");
        println!("   ❌ BAD: use std::rc::Rc; // Rc is not Send");
        println!("   ✅ GOOD: use std::sync::Arc; // Arc is Send\n");
        
        // Good: Using Arc for thread-safe reference counting
        let shared_data = Arc::new(vec!["data1", "data2", "data3"]);
        let data_clone = Arc::clone(&shared_data);
        
        // This can be safely sent across threads
        tokio::spawn(async move {
            println!("   Accessing shared data in spawned task: {:?}", data_clone);
        }).await.map_err(|e| crate::DemoError::Demo(format!("Spawn error: {}", e)))?;
        
        println!("\n2. Future not Send:");
        println!("   ❌ BAD: async fn process(data: &str) -> Result<String>");
        println!("   ✅ GOOD: async fn process(data: String) -> Result<String>\n");
        
        // Good: Taking ownership for Send futures
        let process_data = |data: String| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            Ok::<String, Box<dyn std::error::Error + Send + Sync>>(data.to_uppercase())
        };
        
        let result = process_data("hello".to_string()).await.map_err(|e| crate::DemoError::Demo(format!("Process error: {}", e)))?;
        println!("   Processed data: {}", result);
        
        Ok(())
    }

    pub async fn demonstrate_error_handling(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Common Rust Errors Demo: Error Handling ===\n");
        
        println!("1. Don't unwrap in production code:");
        println!("   ❌ BAD: let value = conn.get(\"key\").await.unwrap();");
        println!("   ✅ GOOD: let value = conn.get(\"key\").await?;");
        println!("   ✅ GOOD: match conn.get(\"key\").await {{ Ok(v) => v, Err(e) => ... }}\n");
        
        // Good: Proper error handling
        match conn.get::<_, Option<String>>("nonexistent_key").await {
            Ok(Some(value)) => println!("   Found value: {}", value),
            Ok(None) => println!("   Key not found (handled gracefully)"),
            Err(e) => println!("   Redis error: {}", e),
        }
        
        println!("\n2. Using Result type and ? operator:");
        // Set a test key
        conn.set::<_, _, ()>("error_test", "test_value").await?;
        
        // Good: Using ? operator for clean error propagation
        let value: String = conn.get("error_test").await?;
        println!("   Successfully retrieved: {}", value);
        
        println!("\n3. Custom error context:");
        println!("   ✅ GOOD: .context(\"Failed to read from Redis\")?");
        
        Ok(())
    }

    pub async fn demonstrate_performance_pitfalls(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Common Rust Errors Demo: Performance ===\n");
        
        println!("1. Unnecessary cloning:");
        println!("   ❌ BAD: fn process(s: String) -> String {{ s.clone() }}");
        println!("   ✅ GOOD: fn process(s: String) -> String {{ s }}");
        println!("   ✅ GOOD: fn process(s: &str) -> String {{ s.to_string() }}\n");
        
        // Good: Avoid unnecessary clones
        let data = "performance_test";
        conn.set::<_, _, ()>("perf_key", data).await?; // No clone needed
        
        println!("2. Efficient string building:");
        println!("   ❌ BAD: result = result + &i.to_string(); // Creates new String");
        println!("   ✅ GOOD: result.push_str(&i.to_string()); // Modifies in place\n");
        
        // Good: Efficient string building
        let mut result = String::with_capacity(100);
        for i in 0..5 {
            use std::fmt::Write;
            write!(&mut result, "item{},", i).unwrap();
        }
        println!("   Efficiently built string: {}", result.trim_end_matches(','));
        
        println!("\n3. Avoiding collect when not needed:");
        println!("   ❌ BAD: vec.iter().map(|x| x*2).collect::<Vec<_>>().iter().sum()");
        println!("   ✅ GOOD: vec.iter().map(|x| x*2).sum()\n");
        
        // Good: Direct sum without intermediate collection
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().map(|x| x * 2).sum();
        println!("   Direct sum result: {}", sum);
        
        Ok(())
    }

    pub async fn cleanup(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        // Clean up test keys
        let test_keys = vec![
            "key1", "key2", "key3",
            "lifetime_test1", "lifetime_test2",
            "type_test", "anno_test", "anno_test2",
            "error_test", "perf_key"
        ];
        
        for key in test_keys {
            let _: std::result::Result<(), _> = conn.del(key).await;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    async fn get_test_client() -> RedisClient {
        RedisClient::new("redis://localhost:6379/15").unwrap()
    }
    
    async fn cleanup_test_keys(client: &RedisClient) {
        let mut conn = client.get_async_connection().await.unwrap();
        let _: String = redis::cmd("FLUSHDB")
            .query_async(&mut conn)
            .await
            .unwrap_or_default();
    }
    
    #[tokio::test]
    async fn test_ownership_errors_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = RustErrorsDemo::new(client.clone());
        let result = demo.demonstrate_ownership_errors().await;
        assert!(result.is_ok());
        
        demo.cleanup().await.unwrap();
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_lifetime_errors_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = RustErrorsDemo::new(client.clone());
        let result = demo.demonstrate_lifetime_errors().await;
        assert!(result.is_ok());
        
        demo.cleanup().await.unwrap();
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_type_errors_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = RustErrorsDemo::new(client.clone());
        let result = demo.demonstrate_type_errors().await;
        assert!(result.is_ok());
        
        demo.cleanup().await.unwrap();
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_async_errors_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = RustErrorsDemo::new(client.clone());
        let result = demo.demonstrate_async_errors().await;
        assert!(result.is_ok());
        
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_error_handling_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = RustErrorsDemo::new(client.clone());
        let result = demo.demonstrate_error_handling().await;
        assert!(result.is_ok());
        
        demo.cleanup().await.unwrap();
        cleanup_test_keys(&client).await;
    }
    
    #[tokio::test]
    async fn test_performance_pitfalls_demo() {
        let client = get_test_client().await;
        cleanup_test_keys(&client).await;
        
        let demo = RustErrorsDemo::new(client.clone());
        let result = demo.demonstrate_performance_pitfalls().await;
        assert!(result.is_ok());
        
        demo.cleanup().await.unwrap();
        cleanup_test_keys(&client).await;
    }
}