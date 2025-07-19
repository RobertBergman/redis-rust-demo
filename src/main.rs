use clap::Parser;
use redis_rust_demo::{RedisClient, Result};
use redis_rust_demo::cli::{Cli, Commands, BasicOperations};
use redis_rust_demo::demos::{BasicOpsDemo, ListDemo, SetDemo, HashDemo, RustErrorsDemo};
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "redis_rust_demo=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Create Redis client
    let redis_client = RedisClient::new(&cli.redis_url)?;
    
    // Execute command
    match cli.command {
        Commands::Ping => {
            info!("Testing Redis connection...");
            match redis_client.ping().await {
                Ok(()) => {
                    println!("✅ Successfully connected to Redis!");
                    println!("Redis URL: {}", cli.redis_url);
                }
                Err(e) => {
                    error!("Failed to connect to Redis: {}", e);
                    println!("❌ Failed to connect to Redis: {}", e);
                    println!("Make sure Redis is running on {}", cli.redis_url);
                }
            }
        }
        Commands::Basic { operation } => {
            match operation {
                BasicOperations::Strings => {
                    let demo = BasicOpsDemo::new(redis_client);
                    demo.string_operations().await?;
                    demo.key_operations().await?;
                }
                BasicOperations::Lists => {
                    let demo = ListDemo::new(redis_client);
                    demo.demonstrate().await?;
                }
                BasicOperations::Sets => {
                    let demo = SetDemo::new(redis_client);
                    demo.demonstrate().await?;
                }
                BasicOperations::Hashes => {
                    let demo = HashDemo::new(redis_client);
                    demo.demonstrate().await?;
                }
            }
        }
        Commands::RustErrors => {
            let demo = RustErrorsDemo::new(redis_client);
            demo.demonstrate_ownership_errors().await?;
            demo.demonstrate_lifetime_errors().await?;
            demo.demonstrate_type_errors().await?;
            demo.demonstrate_async_errors().await?;
            demo.demonstrate_error_handling().await?;
            demo.demonstrate_performance_pitfalls().await?;
            demo.cleanup().await?;
            println!("\n✅ Rust errors demonstration completed!");
        }
    }
    
    Ok(())
}
