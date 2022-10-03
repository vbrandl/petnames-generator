use anyhow::Result;
use ructe::Ructe;
use vergen::{vergen, Config, SemverKind, ShaKind};

fn main() -> Result<()> {
    let mut config = Config::default();
    let git = config.git_mut();
    *git.sha_kind_mut() = ShaKind::Short;
    *git.semver_mut() = true;
    *git.semver_dirty_mut() = Some("-dirty");
    *git.semver_kind_mut() = SemverKind::Lightweight;
    vergen(config)?;

    let mut ructe = Ructe::from_env()?;
    ructe.statics()?.add_files("static")?;
    Ok(ructe.compile_templates("templates")?)
}
