fn main() {
    // cargo pkg version fileds
    println!(
        "CARGO_PKG_VERSION                       : {}",
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "CARGO_PKG_VERSION_MAJOR                 : {}",
        env!("CARGO_PKG_VERSION_MAJOR")
    );
    println!(
        "CARGO_PKG_VERSION_MINOR                 : {}",
        env!("CARGO_PKG_VERSION_MINOR")
    );
    println!(
        "CARGO_PKG_VERSION_PATCH                 : {}",
        env!("CARGO_PKG_VERSION_PATCH")
    );
    println!(
        "CARGO_PKG_VERSION_PRE                   : {}",
        env!("CARGO_PKG_VERSION_PRE")
    );

    // all version fields
    println!(
        "GITVERSION_ASSEMBLY_SEM_FILE_VER        : {}",
        env!("GITVERSION_ASSEMBLY_SEM_FILE_VER")
    );
    println!(
        "GITVERSION_ASSEMBLY_SEM_VER             : {}",
        env!("GITVERSION_ASSEMBLY_SEM_VER")
    );
    println!(
        "GITVERSION_BRANCH_NAME                  : {}",
        env!("GITVERSION_BRANCH_NAME")
    );
    println!(
        "GITVERSION_BUILD_META_DATA              : {}",
        env!("GITVERSION_BUILD_META_DATA")
    );
    println!(
        "GITVERSION_COMMIT_DATE                  : {}",
        env!("GITVERSION_COMMIT_DATE")
    );
    println!(
        "GITVERSION_COMMITS_SINCE_VERSION_SOURCE : {}",
        env!("GITVERSION_COMMITS_SINCE_VERSION_SOURCE")
    );
    println!(
        "GITVERSION_ESCAPED_BRANCH_NAME          : {}",
        env!("GITVERSION_ESCAPED_BRANCH_NAME")
    );
    println!(
        "GITVERSION_FULL_BUILD_META_DATA         : {}",
        env!("GITVERSION_FULL_BUILD_META_DATA")
    );
    println!(
        "GITVERSION_FULL_SEM_VER                 : {}",
        env!("GITVERSION_FULL_SEM_VER")
    );
    println!(
        "GITVERSION_INFORMATIONAL_VERSION        : {}",
        env!("GITVERSION_INFORMATIONAL_VERSION")
    );
    println!(
        "GITVERSION_MAJOR                        : {}",
        env!("GITVERSION_MAJOR")
    );
    println!(
        "GITVERSION_MAJOR_MINOR_PATCH            : {}",
        env!("GITVERSION_MAJOR_MINOR_PATCH")
    );
    println!(
        "GITVERSION_MINOR                        : {}",
        env!("GITVERSION_MINOR")
    );
    println!(
        "GITVERSION_PATCH                        : {}",
        env!("GITVERSION_PATCH")
    );
    println!(
        "GITVERSION_PRE_RELEASE_LABEL            : {}",
        env!("GITVERSION_PRE_RELEASE_LABEL")
    );
    println!(
        "GITVERSION_PRE_RELEASE_LABEL_WITH_DASH  : {}",
        env!("GITVERSION_PRE_RELEASE_LABEL_WITH_DASH")
    );
    println!(
        "GITVERSION_PRE_RELEASE_NUMBER           : {}",
        env!("GITVERSION_PRE_RELEASE_NUMBER")
    );
    println!(
        "GITVERSION_PRE_RELEASE_TAG              : {}",
        env!("GITVERSION_PRE_RELEASE_TAG")
    );
    println!(
        "GITVERSION_PRE_RELEASE_TAG_WITH_DASH    : {}",
        env!("GITVERSION_PRE_RELEASE_TAG_WITH_DASH")
    );
    println!(
        "GITVERSION_SEM_VER                      : {}",
        env!("GITVERSION_SEM_VER")
    );
    println!(
        "GITVERSION_SHA                          : {}",
        env!("GITVERSION_SHA")
    );
    println!(
        "GITVERSION_SHORT_SHA                    : {}",
        env!("GITVERSION_SHORT_SHA")
    );
    println!(
        "GITVERSION_UNCOMMITTED_CHANGES          : {}",
        env!("GITVERSION_UNCOMMITTED_CHANGES")
    );
    println!(
        "GITVERSION_VERSION_SOURCE_SHA           : {}",
        env!("GITVERSION_VERSION_SOURCE_SHA")
    );
    println!(
        "GITVERSION_WEIGHTED_PRE_RELEASE_NUMBER  : {}",
        env!("GITVERSION_WEIGHTED_PRE_RELEASE_NUMBER")
    );
}
