#![allow(unused_variables)]

use celery::task::TaskResult;
use celery::RegularSchedule;
use celery::ScheduledTask;
use celery::SchedulerBackend;
use env_logger::Env;
use exitfailure::ExitFailure;
use std::collections::BinaryHeap;
use tokio::time::Duration;

const QUEUE_NAME: &str = "scheduled";

#[celery::task]
fn add(x: i32, y: i32) -> TaskResult<i32> {
    unimplemented!()
}

#[celery::task]
fn subtract(x: i32, y: i32) -> TaskResult<i32> {
    unimplemented!()
}

struct CustomSchedulerBackend {}

impl CustomSchedulerBackend {
    fn new() -> Self {
        Self {}
    }
}

impl SchedulerBackend for CustomSchedulerBackend {
    fn should_sync(&self) -> bool {
        unimplemented!()
    }

    fn sync(&mut self, scheduled_tasks: &mut BinaryHeap<ScheduledTask>) {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    // Build a `Beat` with a default scheduler backend.
    let mut beat = celery::beat!(
        broker = AMQP { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/my_vhost".into()) },
        task_routes = [
            "*" => QUEUE_NAME,
        ],
    );

    // Build a `Beat` with a custom scheduler backend.
    let custom_backend_beat = celery::beat!(
        broker = AMQP { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/my_vhost".into()) },
        task_routes = [
            "*" => QUEUE_NAME,
        ],
        scheduler_backend = { CustomSchedulerBackend::new() }
    );

    // Add scheduled tasks to the default `Beat` and start it.
    let add_schedule = RegularSchedule::new(Duration::from_secs(1));
    beat.schedule_task(add::new(1, 2), add_schedule);

    let subtract_schedule = RegularSchedule::new(Duration::from_millis(700));
    beat.schedule_task(subtract::new(2, 6), subtract_schedule);

    beat.start().await;

    Ok(())
}
