use crate::{RedisClient, Result};
use redis::AsyncCommands;
use tracing::info;
use std::collections::HashMap;

pub struct ListDemo {
    client: RedisClient,
}

impl ListDemo {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    pub async fn demonstrate(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== List Operations Demo ===\n");
        
        // LPUSH and RPUSH
        println!("1. LPUSH and RPUSH:");
        conn.lpush("mylist", vec!["first", "second"]).await?;
        conn.rpush("mylist", vec!["third", "fourth"]).await?;
        println!("   LPUSH mylist first second");
        println!("   RPUSH mylist third fourth");
        
        // LRANGE
        println!("\n2. LRANGE (view list):");
        let list: Vec<String> = conn.lrange("mylist", 0, -1).await?;
        println!("   LRANGE mylist 0 -1 => {:?}", list);
        
        // LLEN
        println!("\n3. LLEN (list length):");
        let len: usize = conn.llen("mylist").await?;
        println!("   LLEN mylist => {}", len);
        
        // LPOP and RPOP
        println!("\n4. LPOP and RPOP:");
        let left_val: Option<String> = conn.lpop("mylist", None).await?;
        let right_val: Option<String> = conn.rpop("mylist", None).await?;
        println!("   LPOP mylist => {:?}", left_val);
        println!("   RPOP mylist => {:?}", right_val);
        
        let list_after: Vec<String> = conn.lrange("mylist", 0, -1).await?;
        println!("   List after pops: {:?}", list_after);
        
        // LINDEX
        println!("\n5. LINDEX (get by index):");
        let element: Option<String> = conn.lindex("mylist", 0).await?;
        println!("   LINDEX mylist 0 => {:?}", element);
        
        // LINSERT
        println!("\n6. LINSERT:");
        conn.linsert_before("mylist", "third", "inserted").await?;
        let list_inserted: Vec<String> = conn.lrange("mylist", 0, -1).await?;
        println!("   LINSERT mylist BEFORE third inserted");
        println!("   List after insert: {:?}", list_inserted);
        
        // Message Queue Pattern
        println!("\n7. Message Queue Pattern:");
        conn.del("queue:tasks").await?;
        
        // Producer
        println!("   Producer adding tasks:");
        for i in 1..=5 {
            conn.rpush("queue:tasks", format!("task-{}", i)).await?;
            println!("     Added task-{}", i);
        }
        
        // Consumer
        println!("   Consumer processing tasks:");
        while let Some(task) = conn.lpop::<_, Option<String>>("queue:tasks", None).await? {
            println!("     Processing: {}", task);
        }
        
        // BLPOP (blocking pop)
        println!("\n8. BLPOP (blocking pop with timeout):");
        conn.rpush("queue:priority", "urgent-task").await?;
        
        let result: Option<(String, String)> = redis::cmd("BLPOP")
            .arg("queue:priority")
            .arg("queue:normal")
            .arg(2) // 2 second timeout
            .query_async(&mut conn)
            .await?;
        
        if let Some((queue, value)) = result {
            println!("   BLPOP queue:priority queue:normal 2");
            println!("   Received '{}' from queue '{}'", value, queue);
        }
        
        // Clean up
        conn.del("mylist").await?;
        conn.del("queue:tasks").await?;
        conn.del("queue:priority").await?;
        
        info!("List operations demo completed");
        Ok(())
    }
}

pub struct SetDemo {
    client: RedisClient,
}

impl SetDemo {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    pub async fn demonstrate(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Set Operations Demo ===\n");
        
        // SADD
        println!("1. SADD (add members):");
        conn.sadd("fruits", vec!["apple", "banana", "orange"]).await?;
        conn.sadd("fruits", "apple").await?; // Duplicate, won't be added
        conn.sadd("vegetables", vec!["carrot", "broccoli", "spinach"]).await?;
        println!("   SADD fruits apple banana orange");
        println!("   SADD vegetables carrot broccoli spinach");
        
        // SMEMBERS
        println!("\n2. SMEMBERS (get all members):");
        let fruits: Vec<String> = conn.smembers("fruits").await?;
        println!("   SMEMBERS fruits => {:?}", fruits);
        
        // SCARD
        println!("\n3. SCARD (set cardinality):");
        let count: usize = conn.scard("fruits").await?;
        println!("   SCARD fruits => {}", count);
        
        // SISMEMBER
        println!("\n4. SISMEMBER (check membership):");
        let is_member: bool = conn.sismember("fruits", "apple").await?;
        let not_member: bool = conn.sismember("fruits", "potato").await?;
        println!("   SISMEMBER fruits apple => {}", is_member);
        println!("   SISMEMBER fruits potato => {}", not_member);
        
        // SREM
        println!("\n5. SREM (remove members):");
        conn.srem("fruits", "banana").await?;
        let fruits_after: Vec<String> = conn.smembers("fruits").await?;
        println!("   SREM fruits banana");
        println!("   Fruits after removal: {:?}", fruits_after);
        
        // Set operations
        conn.sadd("healthy", vec!["apple", "carrot", "spinach"]).await?;
        
        // SUNION
        println!("\n6. SUNION (union of sets):");
        let union: Vec<String> = conn.sunion(&["fruits", "vegetables"]).await?;
        println!("   SUNION fruits vegetables => {:?}", union);
        
        // SINTER
        println!("\n7. SINTER (intersection):");
        let inter: Vec<String> = conn.sinter(&["fruits", "healthy"]).await?;
        println!("   SINTER fruits healthy => {:?}", inter);
        
        // SDIFF
        println!("\n8. SDIFF (difference):");
        let diff: Vec<String> = conn.sdiff(&["vegetables", "healthy"]).await?;
        println!("   SDIFF vegetables healthy => {:?}", diff);
        
        // Unique visitors pattern
        println!("\n9. Unique Visitors Pattern:");
        let today = "2024-01-15";
        let yesterday = "2024-01-14";
        
        // Simulate visitor IDs
        conn.sadd(format!("visitors:{}", today), vec!["user1", "user2", "user3"]).await?;
        conn.sadd(format!("visitors:{}", yesterday), vec!["user2", "user4", "user5"]).await?;
        
        let today_count: usize = conn.scard(format!("visitors:{}", today)).await?;
        let returning: Vec<String> = conn.sinter(&[
            &format!("visitors:{}", today),
            &format!("visitors:{}", yesterday)
        ]).await?;
        
        println!("   Unique visitors today: {}", today_count);
        println!("   Returning visitors: {:?}", returning);
        
        // SPOP and SRANDMEMBER
        println!("\n10. SPOP and SRANDMEMBER:");
        conn.sadd("lottery", vec!["ticket1", "ticket2", "ticket3", "ticket4"]).await?;
        
        let random: Option<String> = conn.srandmember("lottery").await?;
        println!("   SRANDMEMBER lottery => {:?} (stays in set)", random);
        
        let popped: Option<String> = conn.spop("lottery").await?;
        println!("   SPOP lottery => {:?} (removed from set)", popped);
        
        // Clean up
        conn.del(vec!["fruits", "vegetables", "healthy", "lottery"]).await?;
        conn.del(vec![format!("visitors:{}", today), format!("visitors:{}", yesterday)]).await?;
        
        info!("Set operations demo completed");
        Ok(())
    }
}

pub struct HashDemo {
    client: RedisClient,
}

impl HashDemo {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    pub async fn demonstrate(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        
        println!("\n=== Hash Operations Demo ===\n");
        
        // HSET and HGET
        println!("1. HSET and HGET:");
        conn.hset("user:1000", "name", "Alice Johnson").await?;
        conn.hset("user:1000", "email", "alice@example.com").await?;
        conn.hset("user:1000", "age", 28).await?;
        
        let name: String = conn.hget("user:1000", "name").await?;
        println!("   HSET user:1000 name 'Alice Johnson'");
        println!("   HGET user:1000 name => '{}'", name);
        
        // HMSET (set multiple fields)
        println!("\n2. HMSET (multiple fields):");
        let user_data = vec![
            ("city", "New York"),
            ("country", "USA"),
            ("occupation", "Software Engineer"),
        ];
        conn.hset_multiple("user:1000", &user_data).await?;
        println!("   HMSET user:1000 city 'New York' country 'USA' occupation 'Software Engineer'");
        
        // HGETALL
        println!("\n3. HGETALL (get all fields):");
        let user: HashMap<String, String> = conn.hgetall("user:1000").await?;
        println!("   HGETALL user:1000:");
        for (field, value) in &user {
            println!("     {} => {}", field, value);
        }
        
        // HKEYS and HVALS
        println!("\n4. HKEYS and HVALS:");
        let keys: Vec<String> = conn.hkeys("user:1000").await?;
        let vals: Vec<String> = conn.hvals("user:1000").await?;
        println!("   HKEYS user:1000 => {:?}", keys);
        println!("   HVALS user:1000 => {:?}", vals);
        
        // HEXISTS
        println!("\n5. HEXISTS:");
        let has_email: bool = conn.hexists("user:1000", "email").await?;
        let has_phone: bool = conn.hexists("user:1000", "phone").await?;
        println!("   HEXISTS user:1000 email => {}", has_email);
        println!("   HEXISTS user:1000 phone => {}", has_phone);
        
        // HINCRBY
        println!("\n6. HINCRBY (increment field):");
        conn.hincr("user:1000", "login_count", 1).await?;
        conn.hincr("user:1000", "login_count", 2).await?;
        let count: i64 = conn.hget("user:1000", "login_count").await?;
        println!("   HINCRBY user:1000 login_count 1");
        println!("   HINCRBY user:1000 login_count 2");
        println!("   login_count => {}", count);
        
        // HDEL
        println!("\n7. HDEL (delete fields):");
        conn.hdel("user:1000", "occupation").await?;
        let exists_after: bool = conn.hexists("user:1000", "occupation").await?;
        println!("   HDEL user:1000 occupation");
        println!("   Field exists after deletion: {}", exists_after);
        
        // Shopping Cart Pattern
        println!("\n8. Shopping Cart Pattern:");
        let cart_key = "cart:session123";
        
        // Add items to cart
        conn.hset(cart_key, "product:101", 2).await?; // 2 units
        conn.hset(cart_key, "product:102", 1).await?; // 1 unit
        conn.hset(cart_key, "product:103", 3).await?; // 3 units
        
        println!("   Shopping cart contents:");
        let cart: HashMap<String, i32> = conn.hgetall(cart_key).await?;
        for (product, quantity) in &cart {
            println!("     {} => {} units", product, quantity);
        }
        
        // Update quantity
        conn.hincr(cart_key, "product:101", 1).await?;
        let new_qty: i32 = conn.hget(cart_key, "product:101").await?;
        println!("   Updated product:101 quantity => {} units", new_qty);
        
        // Get total items
        let quantities: Vec<i32> = conn.hvals(cart_key).await?;
        let total_items: i32 = quantities.iter().sum();
        println!("   Total items in cart: {}", total_items);
        
        // Clean up
        conn.del(vec!["user:1000", cart_key]).await?;
        
        info!("Hash operations demo completed");
        Ok(())
    }
}

#[path = "data_structures_tests.rs"]
#[cfg(test)]
mod data_structures_tests;