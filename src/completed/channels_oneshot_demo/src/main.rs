use tokio::{
    sync::oneshot,
    time::{Duration, sleep},
};

#[derive(Debug)]
struct Message {
    message: String,
    sender: String,
}

async fn listener(rx: oneshot::Receiver<Message>) {
    let msg = rx.await.unwrap();
    println!("received message: {} from {}", msg.message, msg.sender);
}

async fn sender(tx: oneshot::Sender<Message>, message: String, delay: Duration) {
    println!("preparing to send message: {}", message);
    sleep(delay).await;
    let message = Message {
        message,
        sender: "SENDER".to_string(),
    };
    println!("sending message: {}", message.message);
    tx.send(message).unwrap();
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    let listener_handle = tokio::spawn(listener(rx));
    let sender_handle = tokio::spawn(sender(tx, "hello".to_string(), Duration::from_secs(2)));

    listener_handle.await.unwrap();
    sender_handle.await.unwrap();
}
