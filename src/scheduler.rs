use crate::config::Job;
use crate::executor::execute_job;
use cron::Schedule;
use std::str::FromStr;
use chrono::Utc;
use tokio::time::{sleep, Duration};
use log::{info, error};

pub async fn run_scheduler(jobs: Vec<Job>) {
    loop {
        let now = Utc::now();

        for job in jobs.clone() {
            // Safely parse the cron expression
            let schedule = match Schedule::from_str(&job.schedule) {
                Ok(s) => s,
                Err(e) => {
                    error!(
                        "Failed to parse cron expression '{}' for job '{}': {}",
                        job.schedule, job.name, e
                    );
                    continue;
                }
            };

            // Check if it's time to run this job
            let next = match schedule.upcoming(Utc).next() {
                Some(t) => t,
                None => {
                    error!(" No upcoming schedule for job '{}'", job.name);
                    continue;
                }
            };

            if next.timestamp() == now.timestamp() {
                info!(" Triggering job '{}'", job.name);
                tokio::spawn(execute_job(job));
            }
        }

        // Sleep before checking again
        sleep(Duration::from_secs(60)).await;
    }
}
