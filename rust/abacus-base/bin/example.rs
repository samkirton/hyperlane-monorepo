use color_eyre::Result;

use abacus_base::{Agent, Settings};

/// An example main function for any agent that implemented Default
async fn _example_main<OA>(settings: Settings) -> Result<()>
where
    OA: Agent<Settings = Settings> + Sized + 'static,
{
    // Instantiate an agent
    let oa = OA::from_settings(settings).await?;
    oa.as_ref()
        .settings
        .tracing
        .start_tracing(oa.metrics().span_duration())?;

    // Run the agent
    oa.run_all(vec![]).await?
}

/// Read settings from the config file and set up reporting and logging based
/// on the settings
#[allow(dead_code)]
fn setup() -> Result<Settings> {
    color_eyre::install()?;

    let settings = Settings::new()?;

    Ok(settings)
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let _settings = setup()?;
    // tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(_example_main(settings))?;

    Ok(())
}