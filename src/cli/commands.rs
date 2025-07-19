use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "redis-demo")]
#[command(about = "Redis Rust Demo Application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, default_value = "redis://localhost:6379")]
    pub redis_url: String,
    
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Basic Redis operations demonstrations")]
    Basic {
        #[command(subcommand)]
        operation: BasicOperations,
    },
    
    #[command(about = "Test Redis connection")]
    Ping,
    
    #[command(about = "Demonstrate common Rust errors and their fixes")]
    RustErrors,
}

#[derive(Subcommand, Debug)]
pub enum BasicOperations {
    #[command(about = "String operations demo")]
    Strings,
    
    #[command(about = "List operations demo")]
    Lists,
    
    #[command(about = "Set operations demo")]
    Sets,
    
    #[command(about = "Hash operations demo")]
    Hashes,
}

#[cfg(test)]
#[path = "commands_tests.rs"]
mod commands_tests;