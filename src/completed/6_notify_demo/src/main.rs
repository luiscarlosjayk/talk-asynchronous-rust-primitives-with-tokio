use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::{self, Duration, Instant, sleep};

async fn notifier(notify: Arc<Notify>) {
    let start = Instant::now();
    println!("Preparing to notify...");
    sleep(Duration::from_secs(2)).await;
    notify.notify_waiters();
    println!(
        "Notification sent!\nTook {:?}ms",
        start.elapsed().as_millis()
    );
}

async fn waiter(notify: Arc<Notify>) {
    let start = Instant::now();
    println!("Waiting for notification...");
    notify.notified().await;
    println!(
        "Notification received!\nTook {:?}ms",
        start.elapsed().as_millis()
    );
}

async fn health_check(notify: Arc<Notify>) {
    let start = Instant::now();
    let mut interval = time::interval(Duration::from_millis(200));
    interval.tick().await; // skip the immediate first tick

    loop {
        tokio::select! {
            _ = interval.tick() => {
                println!("Healthcheck ticked at {:?}ms", start.elapsed().as_millis());
            },
            _ = notify.notified() => {
                println!("Healthcheck: received shutdown notify, exiting");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());

    let notifier_handle = tokio::spawn(notifier(notify.clone()));
    let waiter_handle = tokio::spawn(waiter(notify.clone()));
    let health_check_handle = tokio::spawn(health_check(notify.clone()));

    notifier_handle.await.unwrap();
    waiter_handle.await.unwrap();
    health_check_handle.await.unwrap();
}
