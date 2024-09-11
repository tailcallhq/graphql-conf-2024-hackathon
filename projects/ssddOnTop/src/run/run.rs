use crate::blueprint::Blueprint;
use crate::config::ConfigReader;
use crate::run;

pub async fn run() -> anyhow::Result<()> {
    let config_reader = ConfigReader::init();
    let path = std::env::args().into_iter().collect::<Vec<_>>();
    let path = path.get(1).cloned().unwrap_or({
        let root = env!("CARGO_MANIFEST_DIR");
        format!("{}/schema/schema.graphql", root)
    });

    let config = config_reader.read(path)?;
    let blueprint = Blueprint::try_from(&config)?;
    run::http1::start(blueprint).await?;
    Ok(())
}
