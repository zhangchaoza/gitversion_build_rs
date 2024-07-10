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
pub struct GitVersion {
    #[serde(rename = "Major")]
    pub major: i64,
    #[serde(rename = "Minor")]
    pub minor: i64,
    #[serde(rename = "Patch")]
    pub patch: i64,
    #[serde(rename = "PreReleaseTag")]
    pub pre_release_tag: String,
    #[serde(rename = "PreReleaseTagWithDash")]
    pub pre_release_tag_with_dash: String,
    #[serde(rename = "PreReleaseLabel")]
    pub pre_release_label: String,
    #[serde(rename = "PreReleaseLabelWithDash")]
    pub pre_release_label_with_dash: String,
    #[serde(rename = "PreReleaseNumber")]
    pub pre_release_number: Option<serde_json::Value>,
    #[serde(rename = "WeightedPreReleaseNumber")]
    pub weighted_pre_release_number: i64,
    #[serde(rename = "BuildMetaData")]
    pub build_meta_data: Option<serde_json::Value>,
    #[serde(rename = "BuildMetaDataPadded")]
    pub build_meta_data_padded: String,
    #[serde(rename = "FullBuildMetaData")]
    pub full_build_meta_data: String,
    #[serde(rename = "MajorMinorPatch")]
    pub major_minor_patch: String,
    #[serde(rename = "SemVer")]
    pub sem_ver: String,
    #[serde(rename = "LegacySemVer")]
    pub legacy_sem_ver: String,
    #[serde(rename = "LegacySemVerPadded")]
    pub legacy_sem_ver_padded: String,
    #[serde(rename = "AssemblySemVer")]
    pub assembly_sem_ver: String,
    #[serde(rename = "AssemblySemFileVer")]
    pub assembly_sem_file_ver: String,
    #[serde(rename = "FullSemVer")]
    pub full_sem_ver: String,
    #[serde(rename = "InformationalVersion")]
    pub informational_version: String,
    #[serde(rename = "BranchName")]
    pub branch_name: String,
    #[serde(rename = "EscapedBranchName")]
    pub escaped_branch_name: String,
    #[serde(rename = "Sha")]
    pub sha: String,
    #[serde(rename = "ShortSha")]
    pub short_sha: String,
    #[serde(rename = "NuGetVersionV2")]
    pub nu_get_version_v2: String,
    #[serde(rename = "NuGetVersion")]
    pub nu_get_version: String,
    #[serde(rename = "NuGetPreReleaseTagV2")]
    pub nu_get_pre_release_tag_v2: String,
    #[serde(rename = "NuGetPreReleaseTag")]
    pub nu_get_pre_release_tag: String,
    #[serde(rename = "VersionSourceSha")]
    pub version_source_sha: String,
    #[serde(rename = "CommitsSinceVersionSource")]
    pub commits_since_version_source: i64,
    #[serde(rename = "CommitsSinceVersionSourcePadded")]
    pub commits_since_version_source_padded: String,
    #[serde(rename = "UncommittedChanges")]
    pub uncommitted_changes: i64,
    #[serde(rename = "CommitDate")]
    pub commit_date: String,
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
