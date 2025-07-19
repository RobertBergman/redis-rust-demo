pub mod redis_client;
pub mod error;

pub use redis_client::RedisClient;
pub use error::{DemoError, Result};