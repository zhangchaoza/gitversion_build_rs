use serde::{Deserialize, Serialize};
use std::process::Command;

pub fn get_version() -> Result<GitVersion, String> {
    let version = get_git_version();
    println!("version: {}", version);
    let v: GitVersion = serde_json::from_str(version.as_str())
        .map_err(|e| format!("Get gitversion failed! {e}"))?;
    Ok(v)
}

pub fn inject_version() -> Result<(), String> {
    let v = get_version()?;

    println!(
        "cargo::rustc-env=CARGO_PKG_VERSION={}{}",
        v.major_minor_patch,
        match v.pre_release_label_with_dash.as_str() {
            "" => format!("-{}", v.full_build_meta_data),
            _ => format!(
                "{}.{}",
                v.pre_release_label_with_dash, v.full_build_meta_data
            ),
        }
    );
    println!("cargo::rustc-env=CARGO_PKG_VERSION_MAJOR={}", v.major);
    println!("cargo::rustc-env=CARGO_PKG_VERSION_MINOR={}", v.minor);
    println!("cargo::rustc-env=CARGO_PKG_VERSION_PATCH={}", v.patch);
    println!(
        "cargo::rustc-env=CARGO_PKG_VERSION_PRE={}",
        v.informational_version
    );

    // add all version fileds to env
    println!(
        "cargo::rustc-env=GITVERSION_ASSEMBLY_SEM_FILE_VER={}",
        v.assembly_sem_file_ver
    );
    println!(
        "cargo::rustc-env=GITVERSION_ASSEMBLY_SEM_VER={}",
        v.assembly_sem_ver
    );
    println!("cargo::rustc-env=GITVERSION_BRANCH_NAME={}", v.branch_name);
    println!(
        "cargo::rustc-env=GITVERSION_BUILD_META_DATA={}",
        v.build_meta_data.unwrap_or("".into())
    );
    println!("cargo::rustc-env=GITVERSION_COMMIT_DATE={}", v.commit_date);
    println!(
        "cargo::rustc-env=GITVERSION_COMMITS_SINCE_VERSION_SOURCE={}",
        v.commits_since_version_source
    );
    println!(
        "cargo::rustc-env=GITVERSION_ESCAPED_BRANCH_NAME={}",
        v.escaped_branch_name
    );
    println!(
        "cargo::rustc-env=GITVERSION_FULL_BUILD_META_DATA={}",
        v.full_build_meta_data
    );
    println!(
        "cargo::rustc-env=GITVERSION_FULL_SEM_VER={}",
        v.full_sem_ver
    );
    println!(
        "cargo::rustc-env=GITVERSION_INFORMATIONAL_VERSION={}",
        v.informational_version
    );
    println!("cargo::rustc-env=GITVERSION_MAJOR={}", v.major);
    println!(
        "cargo::rustc-env=GITVERSION_MAJOR_MINOR_PATCH={}",
        v.major_minor_patch
    );
    println!("cargo::rustc-env=GITVERSION_MINOR={}", v.minor);
    println!("cargo::rustc-env=GITVERSION_PATCH={}", v.patch);
    println!(
        "cargo::rustc-env=GITVERSION_PRE_RELEASE_LABEL={}",
        v.pre_release_label
    );
    println!(
        "cargo::rustc-env=GITVERSION_PRE_RELEASE_LABEL_WITH_DASH={}",
        v.pre_release_label_with_dash
    );
    println!(
        "cargo::rustc-env=GITVERSION_PRE_RELEASE_NUMBER={}",
        v.pre_release_number.map_or("".into(), |x| x.to_string())
    );
    println!(
        "cargo::rustc-env=GITVERSION_PRE_RELEASE_TAG={}",
        v.pre_release_tag
    );
    println!(
        "cargo::rustc-env=GITVERSION_PRE_RELEASE_TAG_WITH_DASH={}",
        v.pre_release_tag_with_dash
    );
    println!("cargo::rustc-env=GITVERSION_SEM_VER={}", v.sem_ver);
    println!("cargo::rustc-env=GITVERSION_SHA={}", v.sha);
    println!("cargo::rustc-env=GITVERSION_SHORT_SHA={}", v.short_sha);
    println!(
        "cargo::rustc-env=GITVERSION_UNCOMMITTED_CHANGES={}",
        v.uncommitted_changes
    );
    println!(
        "cargo::rustc-env=GITVERSION_VERSION_SOURCE_SHA={}",
        v.version_source_sha
    );
    println!(
        "cargo::rustc-env=GITVERSION_WEIGHTED_PRE_RELEASE_NUMBER={}",
        v.weighted_pre_release_number
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
    let child = Command::new("dotnet")
        .arg("gitversion")
        .args(["/output", "json"])
        .output();

    let child = if let Ok(child) = child {
        if child.status.success() {
            Ok(String::from_utf8(child.stdout).expect("failed to read stdout"))
        } else {
            Err(format!(
                "Execte `dotnet gitversion` failed! {}",
                String::from_utf8(child.stderr).expect("failed to read stdout")
            ))
        }
    } else {
        Err(format!(
            "Execte `dotnet gitversion` failed! {}",
            child.unwrap_err()
        ))
    };

    child.unwrap_or_else(|e| {
        println!("{}", e);
        println!("Try to execte `gitversion`");
        let child = Command::new("gitversion")
            .args(["/output", "json"])
            .output();

        let child = if let Ok(child) = child {
            if child.status.success() {
                Ok(String::from_utf8(child.stdout).expect("failed to read stdout"))
            } else {
                Err(format!(
                    "Execte `gitversion` failed! {}",
                    String::from_utf8(child.stdout).expect("failed to read stdout")
                ))
            }
        } else {
            Err(format!(
                "Execte `gitversion` failed! {}",
                child.unwrap_err()
            ))
        };

        child.unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_version().unwrap();
    }
}
