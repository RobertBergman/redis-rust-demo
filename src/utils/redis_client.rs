use crate::utils::error::Result;
use redis::{aio::ConnectionManager, Client, ConnectionInfo};
use std::sync::Arc;
use tracing::{debug, info};

#[derive(Clone)]
pub struct RedisClient {
    client: Arc<Client>,
    connection_info: ConnectionInfo,
}

impl RedisClient {
    pub fn new(redis_url: &str) -> Result<Self> {
        let connection_info: ConnectionInfo = redis_url.parse()?;
        let client = Client::open(connection_info.clone())?;
        
        info!("Redis client initialized with URL: {}", redis_url);
        
        Ok(Self {
            client: Arc::new(client),
            connection_info,
        })
    }
    
    pub async fn get_async_connection(&self) -> Result<ConnectionManager> {
        debug!("Creating async connection manager");
        let connection_manager = ConnectionManager::new(self.client.as_ref().clone()).await?;
        Ok(connection_manager)
    }
    
    pub fn get_sync_connection(&self) -> Result<redis::Connection> {
        debug!("Creating sync connection");
        let connection = self.client.get_connection()?;
        Ok(connection)
    }
    
    pub async fn ping(&self) -> Result<()> {
        let mut conn = self.get_async_connection().await?;
        redis::cmd("PING").query_async::<_, ()>(&mut conn).await?;
        info!("Successfully pinged Redis server");
        Ok(())
    }
    
    pub fn get_connection_info(&self) -> &ConnectionInfo {
        &self.connection_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_redis_client_creation_valid_url() {
        let client = RedisClient::new("redis://localhost:6379");
        assert!(client.is_ok());
    }
    
    #[test]
    fn test_redis_client_creation_invalid_url() {
        let client = RedisClient::new("invalid://url");
        assert!(client.is_err());
    }
    
    #[test]
    fn test_redis_client_clone() {
        let client = RedisClient::new("redis://localhost:6379").unwrap();
        let cloned = client.clone();
        assert_eq!(
            client.get_connection_info().addr.to_string(),
            cloned.get_connection_info().addr.to_string()
        );
    }
    
    #[tokio::test]
    async fn test_ping_success() {
        let client = RedisClient::new("redis://localhost:6379").unwrap();
        let result = client.ping().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_async_connection() {
        let client = RedisClient::new("redis://localhost:6379").unwrap();
        let conn = client.get_async_connection().await;
        assert!(conn.is_ok());
    }
    
    #[test]
    fn test_get_sync_connection() {
        let client = RedisClient::new("redis://localhost:6379").unwrap();
        let conn = client.get_sync_connection();
        assert!(conn.is_ok());
    }
    
    #[test]
    fn test_get_connection_info() {
        let client = RedisClient::new("redis://localhost:6379/0").unwrap();
        let info = client.get_connection_info();
        // Check that we have connection info
        assert!(matches!(info.addr, redis::ConnectionAddr::Tcp(_, _)));
    }
    
    #[tokio::test]
    async fn test_connection_with_different_db() {
        let client = RedisClient::new("redis://localhost:6379/2").unwrap();
        let mut conn = client.get_async_connection().await.unwrap();
        
        // Test that we're connected to the right database
        let _: () = redis::cmd("SET")
            .arg("test_key")
            .arg("test_value")
            .query_async(&mut conn)
            .await
            .unwrap();
        
        let result: Option<String> = redis::cmd("GET")
            .arg("test_key")
            .query_async(&mut conn)
            .await
            .unwrap();
        
        assert_eq!(result, Some("test_value".to_string()));
        
        // Clean up
        let _: () = redis::cmd("DEL")
            .arg("test_key")
            .query_async(&mut conn)
            .await
            .unwrap();
    }
}