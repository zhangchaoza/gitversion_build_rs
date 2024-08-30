use serde::{Deserialize, Serialize};
use std::process::Command;

pub fn get_version() -> Result<GitVersion, String> {
    let version = get_git_version();
    let v: GitVersion = serde_json::from_str(version.as_str())
        .map_err(|e| format!("Get gitversion failed! {e}"))?;
    Ok(v)
}

pub fn inject_version() -> Result<(), String> {
    let v = get_version()?;

    println!(
        "cargo:rustc-env=CARGO_PKG_VERSION={}{}",
        v.major_minor_patch,
        match v.pre_release_label_with_dash.as_str() {
            "" => format!("-{}", v.full_build_meta_data),
            _ => format!(
                "{}.{}",
                v.pre_release_label_with_dash, v.full_build_meta_data
            ),
        }
    );
    println!("cargo:rustc-env=CARGO_PKG_VERSION_MAJOR={}", v.major);
    println!("cargo:rustc-env=CARGO_PKG_VERSION_MINOR={}", v.minor);
    println!("cargo:rustc-env=CARGO_PKG_VERSION_PATCH={}", v.patch);
    println!(
        "cargo:rustc-env=CARGO_PKG_VERSION_PRE={}",
        v.informational_version
    );

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GitVersion {
    pub assembly_sem_file_ver: String,
    pub assembly_sem_ver: String,
    pub branch_name: String,
    pub build_meta_data: Option<serde_json::Value>,
    pub commit_date: String,
    pub commits_since_version_source: i64,
    pub escaped_branch_name: String,
    pub full_build_meta_data: String,
    pub full_sem_ver: String,
    pub informational_version: String,
    pub major: i64,
    pub major_minor_patch: String,
    pub minor: i64,
    pub patch: i64,
    pub pre_release_label: String,
    pub pre_release_label_with_dash: String,
    pub pre_release_number: Option<i64>,
    pub pre_release_tag: String,
    pub pre_release_tag_with_dash: String,
    pub sem_ver: String,
    pub sha: String,
    pub short_sha: String,
    pub uncommitted_changes: i64,
    pub version_source_sha: String,
    pub weighted_pre_release_number: i64,
}

fn get_git_version() -> String {
    let child = Command::new("gitversion")
        .args(["/output", "json"])
        .output();
    match child {
        Ok(child) => String::from_utf8(child.stdout).expect("failed to read stdout"),
        Err(err) => {
            panic!("`gitversion` err: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_version().unwrap();
    }
}
