use thiserror::Error;

#[derive(Debug, Error)]
pub enum DemoError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Demo-specific error: {0}")]
    Demo(String),
    
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DemoError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_configuration_error() {
        let error = DemoError::Configuration("Invalid config".to_string());
        assert_eq!(error.to_string(), "Configuration error: Invalid config");
    }
    
    #[test]
    fn test_demo_error() {
        let error = DemoError::Demo("Something went wrong".to_string());
        assert_eq!(error.to_string(), "Demo-specific error: Something went wrong");
    }
    
    #[test]
    fn test_redis_error_conversion() {
        let redis_err = redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "Test error"
        ));
        let demo_err: DemoError = redis_err.into();
        assert!(matches!(demo_err, DemoError::Redis(_)));
        assert!(demo_err.to_string().contains("Redis error"));
    }
    
    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found"
        );
        let demo_err: DemoError = io_err.into();
        assert!(matches!(demo_err, DemoError::Io(_)));
        assert!(demo_err.to_string().contains("IO error"));
    }
    
    #[test]
    fn test_serialization_error_conversion() {
        let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();
        let demo_err: DemoError = json_err.into();
        assert!(matches!(demo_err, DemoError::Serialization(_)));
        assert!(demo_err.to_string().contains("Serialization error"));
    }
    
    #[test]
    fn test_result_type_alias() {
        fn returns_ok() -> Result<i32> {
            Ok(42)
        }
        
        fn returns_err() -> Result<i32> {
            Err(DemoError::Demo("Error".to_string()))
        }
        
        assert_eq!(returns_ok().unwrap(), 42);
        assert!(returns_err().is_err());
    }
}