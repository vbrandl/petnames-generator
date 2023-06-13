use std::num::{NonZeroU8, NonZeroUsize};

#[derive(Clone, Copy)]
pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub semver: &'a str,
}

pub const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_GIT_SHA"),
    semver: env!("VERGEN_GIT_DESCRIBE"),
};

pub const DEFAULT_SEPARATOR: &str = "";
pub const DEFAULT_WORDS_PER_NAME: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
pub const DEFAULT_NUMBER_OF_NAMES: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(6) };
