// src/lib.rs

pub mod cli;
pub mod git;
use crate::cli::{Cli, Commands};
use crate::git::get_commits;
use clap::Parser;
use log::{error, info};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Gitエラー: {0}")]
    GitError(#[from] git2::Error),

    #[error("WakaTimeエラー: {0}")]
    WakaTimeError(#[from] reqwest::Error),

    #[error("日付のパースエラー: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("その他のエラー: {0}")]
    Other(String),
}

pub fn run() -> Result<(), ReportError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key: _,
            datetime,
        } => {
            // datetimeはWakaTime機能用なので無視
            info!("リポジトリパス: {}", repo_path);
            let commits = get_commits(&repo_path)?;

            // リポジトリ名を取得
            let repo = git2::Repository::open(&repo_path)?;
            let repo_name = repo
                .path()
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            // 日報のフォーマット
            println!("# やったこと\n");
            println!("{}:", repo_name);
            for commit in commits {
                println!("- {} ([{}]({}))", commit.message, commit.hash, commit.url);
            }

            // WakaTimeセクションは後で実装
            println!("\n## wakatime\n- typescript: 3 hours");

            Ok(())
        }
    }
}
