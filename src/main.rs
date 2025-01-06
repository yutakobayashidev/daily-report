// src/main.rs

use daily_report::run;

fn main() {
    // CLIの実行
    if let Err(e) = run() {
        eprintln!("エラーが発生しました: {}", e);
        std::process::exit(1);
    }
}
