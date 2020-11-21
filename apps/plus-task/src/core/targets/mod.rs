use crate::error::Error::UnknownBuildTarget;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum BuildTarget {
    LinuxX86,
    LinuxArmV7,
    MacX86,
}

impl BuildTarget {
    pub fn all() -> Vec<BuildTarget> {
        vec![
            BuildTarget::LinuxX86,
            BuildTarget::LinuxArmV7,
            BuildTarget::MacX86,
        ]
    }

    pub fn as_triple(&self) -> &str {
        match self {
            BuildTarget::LinuxX86 => "x86_64-unknown-linux-musl",
            BuildTarget::LinuxArmV7 => "armv7-unknown-linux-musleabihf",
            BuildTarget::MacX86 => "x86_64-apple-darwin",
        }
    }
}

impl FromStr for BuildTarget {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BuildTarget::all()
            .iter()
            .find(|target| target.as_triple() == s)
            .ok_or_else(|| UnknownBuildTarget(s.to_string()))
            .map(|target| *target)
    }
}

pub trait AsBuildTarget {
    fn as_build_target(&self) -> &BuildTarget;
}
