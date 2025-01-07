use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use std::str::FromStr;

/// メールアドレスのバリデーション関数
fn validate_email(val: &str) -> Result<String, String> {
    if val.contains('@') && val.contains('.') {
        Ok(val.to_string())
    } else {
        Err(String::from("有効なメールアドレスを指定してください。"))
    }
}

/// RFC 3339形式の日時のパース関数
fn parse_datetime(val: &str) -> Result<DateTime<Utc>, String> {
    DateTime::from_str(val).map_err(|_| {
        String::from("有効なRFC 3339形式の日時を指定してください。例: 2025-01-07T17:00:00Z")
    })
}

/// 日報自動生成ツール
///
/// Gitコミット履歴とWakaTimeデータを基に日報を生成します。
#[derive(Parser)]
#[command(name = "daily_report")]
#[command(about = "Gitコミット履歴とWakaTimeデータを基に日報を生成します", long_about = None)]
pub struct Cli {
    /// サブコマンド
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 日報を生成します
    Generate {
        /// レポートの対象となるリポジトリのパス（複数指定可、デフォルトはカレントディレクトリ）
        #[arg(short = 'r', long, default_values = &["."])]
        repo_path: Vec<String>,

        /// WakaTimeのAPIキー
        #[arg(short = 'w', long)]
        wakatime_api_key: String,

        /// レポートの開始日時（デフォルトは前日の現地時間17:00）
        #[arg(short = 's', long, value_parser = parse_datetime)]
        since: Option<DateTime<Utc>>,

        /// レポートの終了日時（デフォルトは現在時刻）
        #[arg(short = 'u', long, value_parser = parse_datetime)]
        until: Option<DateTime<Utc>>,

        /// Gitのメールアドレスでフィルタリング
        #[arg(short = 'a', long, value_parser = validate_email)]
        author_email: Option<String>,
    },
}
