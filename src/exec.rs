use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use crate::Error;

pub struct FileLoadingFuture {
    pub contents: Arc<Mutex<Option<Result<Vec<u8>, Error>>>>,
}

impl Future for FileLoadingFuture {
    type Output = Result<Vec<u8>, Error>;

    fn poll(self: Pin<&mut Self>, _context: &mut Context) -> Poll<Self::Output> {
        let mut contents = self.contents.lock().unwrap();
        if let Some(contents) = contents.take() {
            Poll::Ready(contents)
        } else {
            Poll::Pending
        }
    }
}
