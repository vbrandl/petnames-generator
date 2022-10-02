use anyhow::Result;
use ructe::Ructe;
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<()> {
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen(config)?;

    let mut ructe = Ructe::from_env()?;
    ructe.statics()?.add_files("static")?;
    Ok(ructe.compile_templates("templates")?)
}
