pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub version: &'a str,
}

pub const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_GIT_SHA_SHORT"),
    version: env!("CARGO_PKG_VERSION"),
};

pub static DEFAULT_SEPARATOR: &str = "";
pub static DEFAULT_WORDS_PER_NAME: u8 = 2;
pub static DEFAULT_NUMBER_OF_NAMES: usize = 6;
