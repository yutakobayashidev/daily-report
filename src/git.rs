// src/git.rs

use chrono::{DateTime, TimeZone, Utc};
use git2::{Error, Repository};
use regex::Regex;
use std::path::Path;

pub struct CommitInfo {
    pub message: String,
    pub hash: String,
    pub url: String,
    pub date: DateTime<Utc>,
}

pub fn get_commits(
    repo_path: &str,
    author_email: Option<String>,
) -> Result<Vec<CommitInfo>, Error> {
    let repo = Repository::open(Path::new(repo_path))?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut commits = Vec::new();

    // 共同著者を抽出するための正規表現
    let co_author_re = Regex::new(r"(?i)Co-authored-by:\s*.*<([^<>]+)>").unwrap();

    for commit_id in revwalk {
        let oid = commit_id?;
        let commit = repo.find_commit(oid)?;

        // メールアドレスでフィルタリング
        if let Some(ref email) = author_email {
            let mut include_commit = false;

            // 主著者のメールアドレスをチェック
            if let Some(commit_email) = commit.author().email() {
                if commit_email == email {
                    include_commit = true;
                }
            }

            // 共同著者のメールアドレスをチェック
            if !include_commit {
                if let Some(message) = commit.message() {
                    for caps in co_author_re.captures_iter(message) {
                        if let Some(co_author_email) = caps.get(1) {
                            if co_author_email.as_str() == email {
                                include_commit = true;
                                break;
                            }
                        }
                    }
                }
            }

            if !include_commit {
                continue; // 主著者または共同著者に指定されたメールアドレスが含まれていない場合はスキップ
            }
        }

        let message = commit.message().unwrap_or("").to_string();
        let hash = oid.to_string();

        // timestamp_opt を使用
        let commit_time = commit.time();
        let secs = commit_time.seconds();
        let datetime = Utc
            .timestamp_opt(secs, 0) // ナノ秒部分を適切に設定
            .single()
            .ok_or_else(|| git2::Error::from_str("Invalid timestamp"))?;

        // GitHubリポジトリのURLを取得
        let url = get_github_url(&repo)?;

        let short_hash = &hash[..7];
        let commit_url = format!("{}/commit/{}", url, hash);

        commits.push(CommitInfo {
            message,
            hash: short_hash.to_string(),
            url: commit_url,
            date: datetime,
        });
    }

    Ok(commits)
}

fn get_github_url(repo: &Repository) -> Result<String, Error> {
    let remotes = repo.remotes()?;
    for remote_name in remotes.iter().flatten() {
        let remote = repo.find_remote(remote_name)?;
        let url = remote
            .url()
            .ok_or_else(|| git2::Error::from_str("Remote URL not found"))?;

        // SSH形式かHTTPS形式かを判定し、GitHubのHTTPS URLを生成
        if url.starts_with("git@github.com:") {
            // 例: git@github.com:user/repo.git -> https://github.com/user/repo
            let https_url = url
                .replace("git@github.com:", "https://github.com/")
                .trim_end_matches(".git")
                .to_string();
            return Ok(https_url);
        } else if url.starts_with("https://github.com/") {
            let https_url = url.trim_end_matches(".git").to_string();
            return Ok(https_url);
        }
    }

    Err(git2::Error::from_str("GitHub remote URL not found"))
}
