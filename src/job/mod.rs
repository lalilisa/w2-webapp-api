use std::sync::Arc;
use tokio::task;
use tokio_cron_scheduler::{Job, JobScheduler};

mod job_handler;

pub async fn init_jobs() -> Arc<JobScheduler> {
    let scheduler = JobScheduler::new().await.unwrap();

    let job = Job::new_async("0 0 21 * * *", |_uuid, _l| {
        Box::pin(async {
            job_handler::remind_learning().await;
        })
    }).unwrap();

    scheduler.add(job).await.unwrap();

    let scheduler_handle = Arc::new(scheduler);
    let scheduler_clone = Arc::clone(&scheduler_handle);

    task::spawn(async move {
        scheduler_clone.start().await.unwrap();
    });

    scheduler_handle
}