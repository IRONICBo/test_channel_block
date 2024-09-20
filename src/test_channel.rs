use once_cell::sync::Lazy;
use tokio::sync::mpsc;

pub static TASK_MANAGER: Lazy<TaskManager> = Lazy::new(TaskManager::default);

pub struct TaskManager {
    task_sender: TaskSender,
    recv_handle: tokio::task::JoinHandle<String>,
}

impl Default for TaskManager {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel::<String>(32);

        let receiver = TaskReceiver::new(rx);
        let recv_handle = tokio::spawn(receiver.recv());

        let sender = TaskSender::new(tx);
        TaskManager { task_sender: sender, recv_handle: recv_handle }
    }
}

impl TaskManager {
    pub async fn spawn(&self, msg: String) {
        self.task_sender.send(msg).await;
    }
}

pub struct TaskSender {
    tx: mpsc::Sender<String>,
}

impl TaskSender {
    pub fn new(tx: mpsc::Sender<String>) -> TaskSender {
        TaskSender { tx }
    }

    pub async fn send(&self, msg: String) {
        println!("Before sending: {}", msg);
        match self.tx.send(msg.clone()).await {
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
    rx: mpsc::Receiver<String>,
}

impl TaskReceiver {
    pub fn new(rx: mpsc::Receiver<String>) -> TaskReceiver {
        TaskReceiver { rx }
    }

    pub async fn recv(mut self) -> String {
        loop {
            match self.rx.recv().await {
                Some(msg) => {
                    println!("Received: {}", msg);
                }
                None => {
                    println!("Channel closed");
                }
            }
        }
    }
}

#[tokio::test]
async fn test_channel() {
    // console_subscriber::init();
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    TASK_MANAGER.recv_handle.abort();

    let msg = "hello".to_string();
    for _ in 0..5 {
        TASK_MANAGER.spawn(msg.clone()).await;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

#[tokio::test]
async fn test_channel2()
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // This recv task will be dropped by the end of runtime.

    let msg = "hello2".to_string();
    for _ in 0..5 {
        TASK_MANAGER.spawn(msg.clone()).await;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;


// #[tokio::test]
// async fn test_channel3() {
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

//     let msg = "hello3".to_string();
//     for _ in 0..5 {
//         TASK_MANAGER.spawn(msg.clone()).await;
//     }

//     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
// }