use std::sync::LazyLock;
use tokio::sync::OnceCell;
use tokio::time::{Duration, sleep};

// The configuration struct we want to fetch and share.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api_key: String,
    pub api_endpoint: String,
    pub timeout_ms: u64,
}

static CONFIG: LazyLock<OnceCell<AppConfig>> = LazyLock::new(OnceCell::new);

async fn fetch_config_from_api() -> AppConfig {
    println!();
    println!("--- FETCHING CONFIG FROM REMOTE API ---");
    sleep(Duration::from_secs(1)).await;

    let config = AppConfig {
        api_key: "luisk-talks-are-cool".to_string(),
        api_endpoint: "https://luisk.com/v1".to_string(),
        timeout_ms: 5000,
    };

    println!("--- CONFIG FETCHED AND INITIALIZED ---\n");
    config
}

#[tokio::main]
async fn main() {
    println!("Application starting. Multiple tasks will try to get config concurrently.");

    let task1 = tokio::spawn(async {
        println!("[Task 1] Needs config...");
        let config = CONFIG.get_or_init(fetch_config_from_api).await;
        println!("[Task 1] Got config! Endpoint: {}", config.api_endpoint);
    });

    let task2 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        println!("[Task 2] Needs config...");
        let config = CONFIG.get_or_init(fetch_config_from_api).await;
        println!("[Task 2] Got config! API Key: {}", config.api_key);
    });

    let task3 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("[Task 3] Needs config...");
        let config = CONFIG.get_or_init(fetch_config_from_api).await;
        println!("[Task 3] Got config! Timeout: {}", config.timeout_ms);
    });

    let _ = tokio::try_join!(task1, task2, task3);

    println!("\n--- A new task starts much later ---");
    let late_task = tokio::spawn(async {
        println!("[Late Task] I also need the config...");
        let config = CONFIG.get_or_init(fetch_config_from_api).await;
        println!("[Late Task] Got config instantly: {:?}", config);
    });

    let _ = late_task.await;
}
