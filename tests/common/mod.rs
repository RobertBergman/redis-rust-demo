use redis_rust_demo::RedisClient;
use std::sync::Once;

static INIT: Once = Once::new();
static mut TEST_REDIS_URL: Option<String> = None;

pub fn setup() {
    INIT.call_once(|| {
        // Initialize logging for tests
        let _ = tracing_subscriber::fmt()
            .with_env_filter("redis_rust_demo=debug")
            .with_test_writer()
            .try_init();
        
        // Set up test Redis URL
        unsafe {
            TEST_REDIS_URL = Some(
                std::env::var("TEST_REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379/1".to_string())
            );
        }
    });
}

pub fn get_test_redis_url() -> String {
    setup();
    unsafe {
        TEST_REDIS_URL.as_ref().unwrap().clone()
    }
}

pub async fn get_test_client() -> RedisClient {
    let url = get_test_redis_url();
    RedisClient::new(&url).expect("Failed to create test Redis client")
}

pub async fn cleanup_test_keys(client: &RedisClient, pattern: &str) {
    let mut conn = client.get_async_connection().await.unwrap();
    let keys: Vec<String> = redis::cmd("KEYS")
        .arg(pattern)
        .query_async(&mut conn)
        .await
        .unwrap_or_default();
    
    if !keys.is_empty() {
        let _: () = redis::cmd("DEL")
            .arg(&keys)
            .query_async(&mut conn)
            .await
            .unwrap_or_default();
    }
}

#[macro_export]
macro_rules! test_with_redis {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        #[serial_test::serial]
        async fn $name() {
            let client = $crate::common::get_test_client().await;
            $crate::common::cleanup_test_keys(&client, "*test*").await;
            
            let result = $body(client.clone()).await;
            
            $crate::common::cleanup_test_keys(&client, "*test*").await;
            result
        }
    };
}