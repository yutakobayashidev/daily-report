use clap::{Parser, Subcommand};

/// 日報自動生成ツール

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
        /// レポートの対象となるリポジトリのパス（デフォルトはカレントディレクトリ）
        #[arg(short, long, default_value = ".")]
        repo_path: String,

        /// WakaTimeのAPIキー
        #[arg(short, long)]
        wakatime_api_key: String,

        /// レポートの生成日時 (デフォルトは現在時刻、RFC 3339形式)
        #[arg(short, long)]
        datetime: Option<String>,
    },
}
