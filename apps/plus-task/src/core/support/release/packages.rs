use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct PackageSettings {
    pub package_name: PackageName,
    pub on_crates_io: bool,
    pub has_git_tag: bool,
    pub has_github_assets: bool,
}

impl PackageSettings {
    pub fn get(name: PackageName) -> PackageSettings {
        match name {
            PackageName::EnvExtractor => PackageSettings {
                package_name: name,
                on_crates_io: true,
                has_git_tag: true,
                has_github_assets: false,
            },
            PackageName::S3Api => PackageSettings {
                package_name: name,
                on_crates_io: false,
                has_git_tag: true,
                has_github_assets: true,
            },
        }
    }
}

#[derive(Debug, EnumIter, PartialEq)]
pub enum PackageName {
    EnvExtractor,
    S3Api,
}

impl PackageName {
    pub fn as_str(&self) -> &str {
        match self {
            PackageName::EnvExtractor => "env-extractor",
            PackageName::S3Api => "s3api",
        }
    }
}
use crate::error::Error::UnknownPackageName;
use strum::IntoEnumIterator;

impl FromStr for PackageName {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for name in PackageName::iter() {
            if name.as_str() == s {
                return Ok(name);
            }
        }
        Err(UnknownPackageName(s.into()))
    }
}
