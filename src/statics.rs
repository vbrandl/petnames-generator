pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub semver: &'a str,
}

pub const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_GIT_SHA_SHORT"),
    semver: env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT"),
};

pub static DEFAULT_SEPARATOR: &str = "";
pub static DEFAULT_WORDS_PER_NAME: u8 = 2;
pub static DEFAULT_NUMBER_OF_NAMES: usize = 6;
