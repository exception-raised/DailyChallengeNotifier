use std::time::Duration;
use daily_challenge_notifier::entrypoint;
use tokio_cron_scheduler::{JobScheduler, JobSchedulerError, JobBuilder};

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    println!("Welcome to Leetcode Daily Challenge Notifier.");
    let sched = JobScheduler::new().await?;

    let job = JobBuilder::new()
        .with_timezone(chrono_tz::UTC)
        .with_cron_job_type()
        .with_schedule("0 0 * * * *")
        .unwrap()
        .with_run_async(Box::new(|uuid, mut l| {
            Box::pin(async move {
                println!("Starting Job: {:?}", uuid);
                entrypoint().await;
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => println!("Main job next tick: {:?}", ts),
                    _ => println!("Could not get next tick for 2s job"),
                }
            })
        }))
        .build()
        .unwrap();

    sched.add(job).await?;

    sched.start().await?;
    
    loop {
        tokio::time::sleep(Duration::from_secs(100)).await;
    }
}
