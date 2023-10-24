use std::{
    future::Future,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use notify::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

pub struct FileSystemWatcher {
    watcher: RecommendedWatcher,
    receiver: std::sync::Arc<tokio::sync::Notify>,
    // receiver: mpsc::Receiver<Result<notify::Event>>,
}

impl FileSystemWatcher {
    fn new(path: &Path) -> FileSystemWatcher {
        // let (tx, rx) = mpsc::channel(100);
        let notify = std::sync::Arc::new(tokio::sync::Notify::new());
        let notify2 = notify.clone();

        let mut watcher = notify::recommended_watcher(move |res| {
            notify2.notify_one();
            // tx.blocking_send(res)
            //     .expect("Failed to send filesystem notification");
        })
        .unwrap();

        // Add a path to be watched.
        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

        // Create an instance of the custom Future
        FileSystemWatcher {
            watcher,
            receiver: notify,
            // receiver: rx,
        }
    }
}

// impl Future for FileSystemWatcher {
//     type Output = ();

//     fn poll(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//     ) -> Poll<Self::Output> {
//         // Poll the receiver for the next event
//         match self.receiver.poll_recv(cx) {
//             Poll::Ready(Some(Ok(event))) => {
//                 println!("Event: {:?}", event);
//                 Poll::Pending
//             }
//             Poll::Ready(Some(Err(e))) => {
//                 println!("Error: {:?}", e);
//                 Poll::Pending
//             }
//             Poll::Ready(None) => Poll::Ready(()), // Receiver closed, future completed
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }
