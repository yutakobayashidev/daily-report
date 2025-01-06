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

/// 日報自動生成ツールのメインロジック
pub fn run() -> Result<(), ReportError> {
    // CLIの引数を解析
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            repo_path,
            wakatime_api_key,
            datetime,
            author_email,
        } => {
            // ログにリポジトリパスとメールアドレスを記録
            info!("リポジトリパス: {}", repo_path);
            if let Some(ref email) = author_email {
                info!("フィルタリング対象のメールアドレス: {}", email);
            }

            // Gitコミット履歴を取得
            let commits = get_commits(&repo_path, author_email)?;

            // リポジトリ名を取得
            let repo = git2::Repository::open(&repo_path)?;
            let repo_name = repo
                .path()
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            // 日報のフォーマットを生成
            generate_report(&repo_name, &commits);

            // WakaTimeセクションは後で実装
            println!("\n## wakatime\n- typescript: 3 hours");

            Ok(())
        }
    }
}

/// 日報のフォーマットを生成して出力する関数
fn generate_report(repo_name: &str, commits: &[git::CommitInfo]) {
    println!("# やったこと\n");
    println!("{}:", repo_name);
    for commit in commits {
        println!("- {} ([{}]({}))", commit.message, commit.hash, commit.url);
    }
}
