use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub age: Option<u8>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub login_count: u32,
    pub is_active: bool,
}

impl User {
    pub fn new(username: String, email: String, full_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            full_name,
            age: None,
            city: None,
            country: None,
            created_at: Utc::now(),
            last_login: None,
            login_count: 0,
            is_active: true,
        }
    }

    pub fn redis_key(&self) -> String {
        format!("user:{}", self.id)
    }

    pub fn username_index_key(&self) -> String {
        format!("username:{}", self.username)
    }

    pub fn email_index_key(&self) -> String {
        format!("email:{}", self.email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(
            "johndoe".to_string(),
            "john@example.com".to_string(),
            "John Doe".to_string()
        );
        
        assert_eq!(user.username, "johndoe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.full_name, "John Doe");
        assert_eq!(user.login_count, 0);
        assert!(user.is_active);
        assert!(user.age.is_none());
        assert!(user.city.is_none());
        assert!(user.country.is_none());
        assert!(user.last_login.is_none());
    }
    
    #[test]
    fn test_user_redis_key() {
        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "Test User".to_string()
        );
        
        let key = user.redis_key();
        assert!(key.starts_with("user:"));
        assert_eq!(key, format!("user:{}", user.id));
    }
    
    #[test]
    fn test_user_index_keys() {
        let user = User::new(
            "alice".to_string(),
            "alice@example.com".to_string(),
            "Alice Smith".to_string()
        );
        
        assert_eq!(user.username_index_key(), "username:alice");
        assert_eq!(user.email_index_key(), "email:alice@example.com");
    }
    
    #[test]
    fn test_user_serialization() {
        let mut user = User::new(
            "bob".to_string(),
            "bob@example.com".to_string(),
            "Bob Johnson".to_string()
        );
        
        user.age = Some(30);
        user.city = Some("New York".to_string());
        user.country = Some("USA".to_string());
        user.login_count = 5;
        
        // Test serialization
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"username\":\"bob\""));
        assert!(json.contains("\"email\":\"bob@example.com\""));
        assert!(json.contains("\"age\":30"));
        assert!(json.contains("\"city\":\"New York\""));
        
        // Test deserialization
        let deserialized: User = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.username, user.username);
        assert_eq!(deserialized.email, user.email);
        assert_eq!(deserialized.age, user.age);
        assert_eq!(deserialized.city, user.city);
        assert_eq!(deserialized.id, user.id);
    }
    
    #[test]
    fn test_user_clone() {
        let user = User::new(
            "charlie".to_string(),
            "charlie@example.com".to_string(),
            "Charlie Brown".to_string()
        );
        
        let cloned = user.clone();
        assert_eq!(cloned.id, user.id);
        assert_eq!(cloned.username, user.username);
        assert_eq!(cloned.email, user.email);
        assert_eq!(cloned.full_name, user.full_name);
    }
    
    #[test]
    fn test_user_debug() {
        let user = User::new(
            "debug_user".to_string(),
            "debug@example.com".to_string(),
            "Debug User".to_string()
        );
        
        let debug_str = format!("{:?}", user);
        assert!(debug_str.contains("User"));
        assert!(debug_str.contains("debug_user"));
        assert!(debug_str.contains("debug@example.com"));
    }
}