use std::sync::Arc;
use tokio::{
    sync::Semaphore,
    task,
    time::{Duration, sleep},
};

struct Client {
    name: String,
}

impl Client {
    fn new(name: String) -> Self {
        Self { name }
    }

    async fn washing_their_clothes(&self, semaphore: &Semaphore) {
        let _permit = semaphore.acquire().await.unwrap();
        println!("{} is washing.", self.name);
        sleep(Duration::from_secs(1)).await;
        println!("{} has finished washing.", self.name);
    }
}

#[tokio::main]
async fn main() {
    // Since we have 3 washing machines, we create a semaphore with 3 permits.
    let semaphore = Arc::new(Semaphore::new(3));

    let clients = vec![
        Client::new("Client #1".to_string()),
        Client::new("Client #2".to_string()),
        Client::new("Client #3".to_string()),
        Client::new("Client #4".to_string()),
        Client::new("Client #5".to_string()),
        Client::new("Client #6".to_string()),
    ];

    let mut handles = vec![];
    for client in clients {
        let semaphore = semaphore.clone();
        handles.push(task::spawn(async move {
            client.washing_their_clothes(&semaphore).await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
