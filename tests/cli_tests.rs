use chrono::{DateTime, Utc};
use clap::Parser;
use daily_report::cli::{Cli, Commands};

/// ヘルパー関数: DateTime<Utc> を簡単に作成
fn parse_utc_datetime(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}

#[test]
fn test_cli_generate_command_single_repo() {
    let args = vec![
        "daily_report",
        "generate",
        "--repo-path",
        "/path/to/repo1",
        "--wakatime-api-key",
        "test_api_key",
        "--since",
        "2025-01-07T17:00:00Z",
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key,
            since,
            until,
            author_email,
        } => {
            assert_eq!(repo_path, vec!["/path/to/repo1".to_string()]);
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(since.unwrap(), parse_utc_datetime("2025-01-07T17:00:00Z"));
            assert_eq!(until, None);
            assert_eq!(author_email, None); // author_email が指定されていないことを確認
        }
    }
}

#[test]
fn test_cli_generate_command_multiple_repos() {
    let args = vec![
        "daily_report",
        "generate",
        "--repo-path",
        "/path/to/repo1",
        "--repo-path",
        "/path/to/repo2",
        "--wakatime-api-key",
        "test_api_key",
        "--since",
        "2025-01-07T17:00:00Z",
        "--until",
        "2025-01-08T17:00:00Z",
        "--author-email",
        "user@example.com",
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key,
            since,
            until,
            author_email,
        } => {
            assert_eq!(
                repo_path,
                vec!["/path/to/repo1".to_string(), "/path/to/repo2".to_string()]
            );
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(since.unwrap(), parse_utc_datetime("2025-01-07T17:00:00Z"));
            assert_eq!(until.unwrap(), parse_utc_datetime("2025-01-08T17:00:00Z"));
            assert_eq!(author_email, Some("user@example.com".to_string())); // author_email が指定されていることを確認
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
            since,
            until,
            author_email,
        } => {
            assert_eq!(repo_path, vec![".".to_string()]);
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(since, None); // since が指定されていないことを確認
            assert_eq!(until, None); // until が指定されていないことを確認
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
        "/path/to/repo1",
        "--repo-path",
        "/path/to/repo2",
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
            since,
            until,
            author_email,
        } => {
            assert_eq!(
                repo_path,
                vec!["/path/to/repo1".to_string(), "/path/to/repo2".to_string()]
            );
            assert_eq!(wakatime_api_key, "test_api_key");
            assert_eq!(since, None); // since が指定されていない場合は None
            assert_eq!(until, None); // until が指定されていない場合は None
            assert_eq!(author_email, Some("user@example.com".to_string())); // author_email が指定されていることを確認
        }
    }
}
