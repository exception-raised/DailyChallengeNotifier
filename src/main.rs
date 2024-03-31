extern crate cronjob;
use cronjob::CronJob;

fn main() {
    let mut cron = CronJob::new("Test Cron", on_cron);
    cron.seconds("0");
    cron.minutes("0");
    cron.hours("0");
    cron.offset(0); // UTC

    cron.start_job();
}
    
// Our cronjob handler
fn on_cron(name: &str) {
    println!("{}: It's time!", name);
}
