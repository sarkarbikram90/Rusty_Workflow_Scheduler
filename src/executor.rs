use std::process::Command;
use tokio::time::{timeout, Duration};
use crate::config::Job;
use log::{info, error};

pub async fn execute_job(job: Job) {
    info!("Running job: {}", job.name);

    let task = tokio::task::spawn_blocking(move || {
        Command::new("sh")
            .arg("-c")
            .arg(&job.command)
            .output()
    });

    match timeout(Duration::from_secs(job.timeout), task).await {
        Ok(join_result) => match join_result {
            Ok(exec_result) => match exec_result {
                Ok(output) => {
                    if output.status.success() {
                        info!(" Job '{}' completed successfully", job.name);
                        info!(" STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        error!(
                            " Job '{}' failed with exit code {:?}",
                            job.name,
                            output.status.code()
                        );
                        error!(
                            " STDERR:\n{}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => error!(" Command execution failed for '{}': {}", job.name, e),
            },
            Err(e) => error!(" Failed to join spawned task for '{}': {}", job.name, e),
        },
        Err(_) => error!(" Job '{}' timed out after {} seconds", job.name, job.timeout),
    }
}
