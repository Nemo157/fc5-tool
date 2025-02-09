use eyre::{bail, Error};

use crate::data::World;

#[culpa::throws]
#[tracing::instrument(name = "randomize", skip_all)]
pub(super) fn run(world: &World) {
    let mut level = world.level()?;
    let Some(fastnbt::Value::Compound(data)) = level.get_mut("Data") else {
        bail!("bad Data")
    };
    let Some(fastnbt::Value::Compound(settings)) = data.get_mut("WorldGenSettings") else {
        bail!("bad WorldGenSettings")
    };
    let Some(fastnbt::Value::Long(seed)) = settings.get_mut("seed") else {
        bail!("bad seed")
    };
    let _guard = tracing::info_span!("from", old.seed = %seed).entered();
    *seed = rand::random();
    let _guard = tracing::info_span!("to", new.seed = %seed).entered();
    world.save_level(&level)?;
    tracing::info!("Randomized seed");
}
