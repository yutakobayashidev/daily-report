// tests/cli_tests.rs

use clap::Parser;
use daily_report::cli::{Cli, Commands};

#[test]
fn test_cli_generate_command() {
    let args = vec![
        "daily_report",
        "generate",
        "--repo-path",
        "/path/to/repo",
        "--wakatime-api-key",
        "test_api_key",
        "--datetime",
        "2025-01-07T17:00:00Z",
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key,
            datetime,
            author_email,
        } => {
            assert_eq!(repo_path, "/path/to/repo");
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(datetime, Some("2025-01-07T17:00:00Z".to_string()));
            assert_eq!(author_email, None); // author_email が指定されていないことを確認
        }
    }
}

#[test]
fn test_cli_generate_command_defaults() {
    let args = vec![
        "daily_report",
        "generate",
        "--wakatime-api-key",
        "test_api_key",
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key,
            datetime,
            author_email,
        } => {
            assert_eq!(repo_path, ".");
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(datetime, None);
            assert_eq!(author_email, None); // author_email が指定されていないことを確認
        }
    }
}

#[test]
fn test_cli_generate_command_with_author_email() {
    let args = vec![
        "daily_report",
        "generate",
        "--repo-path",
        "/path/to/repo",
        "--wakatime-api-key",
        "test_api_key",
        "--author-email",
        "user@example.com",
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key,
            datetime,
            author_email,
        } => {
            assert_eq!(repo_path, "/path/to/repo");
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(datetime, None); // datetime が指定されていない場合は None
            assert_eq!(author_email, Some("user@example.com".to_string())); // author_email が指定されていることを確認
        }
    }
}

#[test]
fn test_cli_generate_command_with_all_options() {
    let args = vec![
        "daily_report",
        "generate",
        "--repo-path",
        "/path/to/repo",
        "--wakatime-api-key",
        "test_api_key",
        "--datetime",
        "2025-01-07T17:00:00Z",
        "--author-email",
        "user@example.com",
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key,
            datetime,
            author_email,
        } => {
            assert_eq!(repo_path, "/path/to/repo");
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(datetime, Some("2025-01-07T17:00:00Z".to_string()));
            assert_eq!(author_email, Some("user@example.com".to_string()));
        }
    }
}
