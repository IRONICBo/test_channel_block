use once_cell::sync::Lazy;
use flume::{self, Receiver, Sender};
use tokio::runtime::Runtime;

pub static TASK_MANAGER: Lazy<TaskManager> = Lazy::new(TaskManager::default);

pub struct TaskManager {
    task_sender: TaskSender,
    recv_handle: tokio::task::JoinHandle<()>,
    recv_runtime: Runtime,
}

impl Default for TaskManager {
    fn default() -> Self {
        let (tx, rx) = flume::unbounded::<String>();

        let recv_runtime = Runtime::new().unwrap();
        let recv_runtime_clone = recv_runtime.handle().clone();

        let recv_handle: tokio::task::JoinHandle<()> = recv_runtime.spawn(async move {
            let receiver = TaskReceiver::new(rx);
            receiver.recv().await;
        });

        let sender = TaskSender::new(tx);
        TaskManager {
            task_sender: sender,
            recv_handle: recv_handle,
            recv_runtime,  // 持有 Runtime
        }
    }
}

impl TaskManager {
    pub async fn spawn(&self, msg: String) {
        self.task_sender.send(msg).await;
    }
}

pub struct TaskSender {
    tx: Sender<String>,
}

impl TaskSender {
    pub fn new(tx: Sender<String>) -> TaskSender {
        TaskSender { tx }
    }

    pub async fn send(&self, msg: String) {
        println!("Before sending: {}", msg);
        match self.tx.send_async(msg.clone()).await {
            Ok(_) => {
                println!("After sending done: {}", msg);
            }
            Err(e) => {
                eprintln!("{} Error sending: {:?}", msg, e);
            }
        }
    }
}

pub struct TaskReceiver {
    rx: Receiver<String>,
}

impl TaskReceiver {
    pub fn new(rx: Receiver<String>) -> TaskReceiver {
        TaskReceiver { rx }
    }

    pub async fn recv(self) {
        loop {
            match self.rx.recv_async().await {
                Ok(msg) => {
                    println!("Received: {}", msg);
                }
                Err(e) => {
                    eprintln!("Error receiving: {:?}", e);
                    break;
                }
            }
        }
    }
}

#[tokio::test]
async fn test_multi_runtime_channel() {
    // console_subscriber::init();

    let msg = "hello".to_string();
    for _ in 0..5 {
        TASK_MANAGER.spawn(msg.clone()).await;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

#[tokio::test]
async fn test_multi_runtime_channel2() {

    let msg = "hello2".to_string();
    for _ in 0..5 {
        TASK_MANAGER.spawn(msg.clone()).await;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

#[tokio::test]
async fn test_multi_runtime_channel3() {

    let msg = "hello3".to_string();
    for _ in 0..5 {
        TASK_MANAGER.spawn(msg.clone()).await;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
