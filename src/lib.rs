pub mod cli;
pub mod git;

use crate::cli::{Cli, Commands};
use crate::git::get_commits;
use chrono::{DateTime, Local, Utc};
use clap::Parser;
use log::{error, info};
use regex::Regex;
use std::path::Path;
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

            // 各リポジトリからコミットを取得
            let mut all_commits = Vec::new();
            let mut base_repo_urls = Vec::new();

            for path in &repo_path {
                info!("リポジトリパス: {}", path);

                // Gitリポジトリを開く
                let repo = git2::Repository::open(path)?;
                // リポジトリのベースURLを取得
                let base_repo_url = git::get_github_url(&repo)?;
                base_repo_urls.push((path.clone(), base_repo_url));

                // Gitコミット履歴を取得
                let commits =
                    get_commits(path, author_email.clone(), since_datetime, until_datetime)?;
                all_commits.push((path.clone(), commits));
            }

            // 日報のフォーマットを生成
            generate_report(&base_repo_urls, &all_commits);

            // WakaTimeセクションは後で実装
            println!("\n## wakatime\n- typescript: 3 hours");

            Ok(())
        }
    }
}

/// 日報のフォーマットを生成して出力する関数
fn generate_report(
    base_repo_urls: &Vec<(String, String)>,
    all_commits: &Vec<(String, Vec<git::CommitInfo>)>,
) {
    println!("# やったこと\n");

    let mut first = true; // 最初のリポジトリかどうかを追跡するフラグ

    for (repo_path, commits) in all_commits {
        // コミットが存在しない場合はスキップ
        if commits.is_empty() {
            continue;
        }

        // 最初のリポジトリ以外では空白行を挿入
        if !first {
            println!();
        }
        first = false;

        // ベースURLを取得
        let base_url = base_repo_urls
            .iter()
            .find(|(path, _)| path == repo_path)
            .map(|(_, url)| url.as_str()) // &String を &str に変換
            .unwrap_or("https://github.com/unknown/repo"); // &str を使用

        // リポジトリ名を取得
        let repo_name = Path::new(repo_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");

        println!("{}:", repo_name);
        for commit in commits {
            // チケット番号をリンクに変換
            let processed_message = replace_ticket_numbers(&commit.message, base_url);
            println!(
                "- {} ([{}]({}))",
                processed_message, commit.hash, commit.url
            );
        }
    }
}

/// コミットメッセージ内のチケット番号（例: #141）をMarkdownリンクに置換する関数
fn replace_ticket_numbers(message: &str, base_repo_url: &str) -> String {
    let re = Regex::new(r"#(\d+)").unwrap();
    re.replace_all(message, |caps: &regex::Captures| {
        let ticket_number = &caps[1];
        format!(
            "[#{}]({}/issues/{})",
            ticket_number, base_repo_url, ticket_number
        )
    })
    .to_string()
}
