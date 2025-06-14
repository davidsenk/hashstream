use std::sync::{Arc, Condvar, Mutex};

pub struct ReturnChannel<T> {
    value: Mutex<Option<T>>,
    condvar: Condvar,
}

impl<T> ReturnChannel<T> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> (ReturnChannelSender<T>, ReturnChannelReceiver<T>) {
        let inner = ReturnChannel {
            value: Mutex::new(None),
            condvar: Condvar::new(),
        };

        let arc = Arc::new(inner);

        let ret_tx = ReturnChannelSender(arc.clone());
        let ret_rx = ReturnChannelReceiver(arc);

        (ret_tx, ret_rx)
    }
}

pub struct ReturnChannelReceiver<T>(Arc<ReturnChannel<T>>);
pub struct ReturnChannelSender<T>(Arc<ReturnChannel<T>>);

impl<T> ReturnChannelReceiver<T> {
    pub fn receive(self) -> T {
        let mut val = self.0.value.lock().unwrap();

        while val.is_none() {
            val = self.0.condvar.wait(val).unwrap()
        }

        let replace = val.take();

        replace.unwrap()
    }
}

impl<T> ReturnChannelSender<T> {
    pub fn send(self, val: T) {
        let mut lock = self.0.value.lock().unwrap();

        *lock = Some(val);

        self.0.condvar.notify_all();
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test_channel_single_thread() {
        let (tx, rx) = ReturnChannel::new();

        tx.send(42);

        let rx = rx.receive();

        assert_eq!(rx, 42);
    }

    #[test]
    fn test_channel_multi_thread() {
        let (tx, rx) = ReturnChannel::new();

        std::thread::spawn(|| {
            tx.send(42);
        });

        let rx = rx.receive();

        assert_eq!(rx, 42);
    }
}
