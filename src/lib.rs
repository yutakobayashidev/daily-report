pub mod cli;
pub mod git;

use crate::cli::{Cli, Commands};
use crate::git::get_commits;
use chrono::{DateTime, Local, Utc};
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
            since,
            until,
            author_email,
        } => {
            // ログにリポジトリパスとメールアドレスを記録
            info!("リポジトリパス: {}", repo_path);
            if let Some(ref email) = author_email {
                info!("フィルタリング対象のメールアドレス: {}", email);
            }

            // Gitリポジトリを開く
            let repo = git2::Repository::open(&repo_path)?;
            // リポジトリのベースURLを取得
            let base_repo_url = git::get_github_url(&repo)?;

            // 開始日時の設定
            let since_datetime: DateTime<Utc> = if let Some(since_dt) = since {
                since_dt
            } else {
                // デフォルト: 前日の現地時間17:00をUTCに変換
                let local_now = Local::now();
                let local_since = local_now - chrono::Duration::days(1);
                let local_since_naive = local_since
                    .date_naive()
                    .and_hms_opt(17, 0, 0)
                    .ok_or_else(|| ReportError::Other("無効な日時".to_string()))?;
                // NaiveDateTime を Local タイムゾーンに結合し、UTC に変換
                let local_since_datetime = local_since_naive
                    .and_local_timezone(Local)
                    .single()
                    .ok_or_else(|| ReportError::Other("タイムゾーン変換エラー".to_string()))?;
                local_since_datetime.with_timezone(&Utc)
            };

            // 終了日時の設定
            let until_datetime: DateTime<Utc> = if let Some(until_dt) = until {
                until_dt
            } else {
                // デフォルト: 現在時刻
                Utc::now()
            };

            info!("開始日時: {}", since_datetime);
            info!("終了日時: {}", until_datetime);

            // Gitコミット履歴を取得
            let commits = get_commits(&repo_path, author_email, since_datetime, until_datetime)?;

            // リポジトリ名を取得
            let repo_name = repo
                .path()
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            // 日報のフォーマットを生成
            generate_report(&repo_name, &base_repo_url, &commits);

            // WakaTimeセクションは後で実装
            println!("\n## wakatime\n- typescript: 3 hours");

            Ok(())
        }
    }
}

/// 日報のフォーマットを生成して出力する関数
fn generate_report(repo_name: &str, base_repo_url: &str, commits: &[git::CommitInfo]) {
    println!("# やったこと\n");
    println!("{}:", repo_name);
    for commit in commits {
        // チケット番号をリンクに変換
        let processed_message = replace_ticket_numbers(&commit.message, base_repo_url);
        println!(
            "- {} ([{}]({}))",
            processed_message, commit.hash, commit.url
        );
    }
}

/// コミットメッセージ内のチケット番号（例: #141）をMarkdownリンクに置換する関数
fn replace_ticket_numbers(message: &str, base_repo_url: &str) -> String {
    let re = regex::Regex::new(r"#(\d+)").unwrap();
    re.replace_all(message, |caps: &regex::Captures| {
        let ticket_number = &caps[1];
        format!(
            "[#{}]({}/issues/{})",
            ticket_number, base_repo_url, ticket_number
        )
    })
    .to_string()
}
