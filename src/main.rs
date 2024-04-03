
use cronjob::CronJob;
use daily_challenge_notifier::load_env_variables;
use daily_challenge_notifier::types::Config;


#[tokio::main]
async fn main() {
    let config: Config = load_env_variables();

    println!("Token: {}", config.smtp_username);

    //let mut cron = CronJob::new("Test Cron", on_cron);
    //cron.seconds("0");
    //cron.minutes("0");
    //cron.hours("0");
    //cron.offset(0); // UTC

    //cron.start_job();

    daily_challenge_notifier::entrypoint(config).await;
}
    