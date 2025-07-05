mod config;
mod scheduler;
mod executor;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let config = config::load_config("jobs.yaml")?;
    log::info!(" Scheduler started with {} job(s)", config.jobs.len());
    scheduler::run_scheduler(config.jobs).await;
    Ok(())
}
