pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub version: &'a str,
}

pub const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_GIT_SHA_SHORT"),
    version: env!("CARGO_PKG_VERSION"),
};
