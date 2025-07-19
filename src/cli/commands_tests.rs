#[cfg(test)]
mod tests {
    use super::super::*;
    use clap::CommandFactory;
    
    #[test]
    fn test_cli_creation() {
        let cli = Cli::command();
        assert_eq!(cli.get_name(), "redis-demo");
    }
    
    #[test]
    fn test_cli_parsing_ping() {
        let args = vec!["redis-demo", "ping"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(cli.command, Commands::Ping));
        assert_eq!(cli.redis_url, "redis://localhost:6379");
        assert!(!cli.verbose);
    }
    
    #[test]
    fn test_cli_parsing_with_options() {
        let args = vec!["redis-demo", "--redis-url", "redis://custom:6380", "--verbose", "ping"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(cli.command, Commands::Ping));
        assert_eq!(cli.redis_url, "redis://custom:6380");
        assert!(cli.verbose);
    }
    
    #[test]
    fn test_cli_parsing_basic_strings() {
        let args = vec!["redis-demo", "basic", "strings"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Basic { operation } => {
                assert!(matches!(operation, BasicOperations::Strings));
            }
            _ => panic!("Expected Basic command"),
        }
    }
    
    #[test]
    fn test_cli_parsing_basic_lists() {
        let args = vec!["redis-demo", "basic", "lists"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Basic { operation } => {
                assert!(matches!(operation, BasicOperations::Lists));
            }
            _ => panic!("Expected Basic command"),
        }
    }
    
    #[test]
    fn test_cli_parsing_basic_sets() {
        let args = vec!["redis-demo", "basic", "sets"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Basic { operation } => {
                assert!(matches!(operation, BasicOperations::Sets));
            }
            _ => panic!("Expected Basic command"),
        }
    }
    
    #[test]
    fn test_cli_parsing_basic_hashes() {
        let args = vec!["redis-demo", "basic", "hashes"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Basic { operation } => {
                assert!(matches!(operation, BasicOperations::Hashes));
            }
            _ => panic!("Expected Basic command"),
        }
    }
    
    #[test]
    fn test_basic_operations_debug() {
        let op = BasicOperations::Strings;
        let debug_str = format!("{:?}", op);
        assert_eq!(debug_str, "Strings");
    }
    
    #[test]
    fn test_cli_parsing_rust_errors() {
        let args = vec!["redis-demo", "rust-errors"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(cli.command, Commands::RustErrors));
    }
}