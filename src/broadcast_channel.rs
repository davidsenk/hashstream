use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, TryLockError, TryLockResult};

struct Inner<T> {
    queue: Arc<Mutex<VecDeque<Arc<T>>>>,
    condvar: Condvar,
}

#[derive(Debug, PartialEq)]
enum TrySendResult {
    OK,
    Closed,
    WouldBlock,
}

#[derive(Debug, PartialEq)]
enum TryRecvResult<T> {
    OK(Arc<T>),
    Closed,
    WouldBlock,
}

impl<T> Inner<T> {
    fn new() -> Inner<T> {
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        Inner {
            queue,
            condvar: Condvar::new(),
        }
    }

    fn try_send_one(&self, item: Arc<T>) -> TrySendResult {
        match self.queue.try_lock() {
            Ok(mut lock) => {
                lock.push_back(item.clone());
                self.condvar.notify_all();
                TrySendResult::OK
            }
            Err(err) => {
                match err {
                    TryLockError::Poisoned(err) => {
                        // dropping poisoned receiver, receiving end panic / closed
                        TrySendResult::Closed
                    }
                    TryLockError::WouldBlock => {
                        // feels like a silly arc clone, but compiler can't figure out
                        // that this is actually ok to not clone
                        TrySendResult::WouldBlock
                    }
                }
            }
        }
    }

    /// If `false` is returned, receiving side has been closed
    /// Blocks until able to send, or receiving side closes
    fn send_one(&self, item: Arc<T>) -> bool {
        let lock = self.queue.lock();

        match lock {
            Ok(mut lock) => {
                lock.push_back(item);
                self.condvar.notify_all();
                true
            }
            Err(err) => false,
        }
    }

    /// If `false` is returned, receiving side has been closed
    /// Blocks until able to send, or receiving side closes
    fn send_many(&self, items: Vec<Arc<T>>) -> bool {
        let lock = self.queue.lock();

        match lock {
            Ok(mut lock) => {
                for item in items {
                    lock.push_back(item)
                }
                self.condvar.notify_all();
                true
            }
            Err(err) => false,
        }
    }

    fn try_send_many(&self, items: Vec<Arc<T>>) -> TrySendResult {
        let lock = self.queue.try_lock();

        match lock {
            Ok(mut lock) => {
                for item in items {
                    lock.push_back(item);
                }

                self.condvar.notify_all();
                TrySendResult::OK
            }
            Err(err) => match err {
                TryLockError::Poisoned(err) => TrySendResult::Closed,
                TryLockError::WouldBlock => TrySendResult::WouldBlock,
            },
        }
    }

    pub fn try_recv_one(&self) -> TryRecvResult<T> {
        let lock = self.queue.try_lock();

        match lock {
            Ok(mut lock) => {
                let val = lock.pop_front();

                match val {
                    None => TryRecvResult::WouldBlock,
                    Some(val) => TryRecvResult::OK(val),
                }
            }
            Err(err) => match err {
                TryLockError::Poisoned(err) => TryRecvResult::Closed,
                TryLockError::WouldBlock => TryRecvResult::WouldBlock,
            },
        }
    }

    /// If `None` is returned, sending side has been closed
    /// Blocks until receipt
    pub fn recv_one(&self) -> Option<Arc<T>> {
        let lock = self.queue.lock();

        match lock {
            Ok(mut lock) => {
                while lock.is_empty() {
                    //lock = self.condvar.wait(lock).unwrap();
                    lock = match self.condvar.wait(lock) {
                        Ok(lock) => lock,
                        Err(err) => {
                            return None;
                        }
                    }
                }
                assert!(!lock.is_empty());
                lock.pop_front()
            }
            Err(err) => None,
        }
    }
}

pub struct BroadcastReceiever<T> {
    inner: Arc<Inner<T>>,
}

impl<T> BroadcastReceiever<T> {
    fn new(inner: Arc<Inner<T>>) -> Self {
        BroadcastReceiever { inner }
    }

    /// Blocks until there is an item to receive,
    /// if `None` is returned, sending side has closed
    pub fn recv_one(&self) -> Option<Arc<T>> {
        self.inner.recv_one()
    }

    pub fn try_recv_one(&self) -> TryRecvResult<T> {
        self.inner.try_recv_one()
    }
}

pub struct BroadcastChannel<T> {
    receivers: Vec<Arc<Inner<T>>>,
}

impl<T> BroadcastChannel<T> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Self {
        BroadcastChannel {
            receivers: Vec::new(),
        }
    }

    pub fn new_receiver(&mut self) -> BroadcastReceiever<T> {
        let inner = Arc::new(Inner::new());
        let receiver = BroadcastReceiever::new(inner.clone());

        self.receivers.push(inner);

        receiver
    }

    /// Return value is number of broadcast receivers `item` was sent to
    pub fn send(&mut self, item: Arc<T>) -> usize {
        let mut sent_count = 0;

        let mut receivers_pending = VecDeque::with_capacity(self.receivers.len());

        while let Some(next) = self.receivers.pop() {
            receivers_pending.push_back(next);
        }

        while let Some(next) = receivers_pending.pop_front() {
            match next.try_send_one(item.clone()) {
                TrySendResult::OK => {
                    // receiver good, still receiving messages, return to receivers vec
                    self.receivers.push(next);
                    sent_count += 1;
                }
                TrySendResult::Closed => {
                    // if broadcast receiver is closed, drop it
                    continue;
                }
                TrySendResult::WouldBlock => {
                    // receiver not ready for next message, return to pending queue
                    receivers_pending.push_back(next);
                }
            }
        }

        sent_count
    }

    /// Return value is number of receivers `item` was sent to
    pub fn send_many(&mut self, item: Vec<Arc<T>>) -> usize {
        let mut sent_count = 0;

        let mut receivers_pending = VecDeque::with_capacity(self.receivers.len());

        while let Some(next) = self.receivers.pop() {
            receivers_pending.push_back(next);
        }

        while let Some(next) = receivers_pending.pop_front() {
            match next.try_send_many(item.clone()) {
                TrySendResult::OK => {
                    // receiver good, still receiving messages, return to receivers vec
                    self.receivers.push(next);
                    sent_count += 1;
                }
                TrySendResult::Closed => {
                    // if broadcast receiver is closed, drop it
                    continue;
                }
                TrySendResult::WouldBlock => {
                    // receiver not ready for next message, return to pending queue
                    receivers_pending.push_back(next);
                }
            }
        }

        sent_count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_single_thread() {
        let mut tx = BroadcastChannel::<usize>::new();

        let rx1 = tx.new_receiver();
        let rx2 = tx.new_receiver();

        let rxc = tx.send(Arc::new(42));

        assert_eq!(rxc, 2);

        assert_eq!(*rx1.recv_one().unwrap(), 42);
        assert_eq!(*rx2.recv_one().unwrap(), 42);

        assert_eq!(rx1.try_recv_one(), TryRecvResult::WouldBlock);
        assert_eq!(rx2.try_recv_one(), TryRecvResult::WouldBlock);

        let rxc = tx.send(Arc::new(69));

        assert_eq!(rxc, 2);

        assert_eq!(*rx1.recv_one().unwrap(), 69);
        assert_eq!(*rx2.recv_one().unwrap(), 69);
    }

    #[test]
    fn test_multi_thread() {
        let mut tx = BroadcastChannel::<usize>::new();

        let mut rx = Vec::new();

        for _ in 0..12 {
            rx.push(tx.new_receiver());
        }

        std::thread::spawn(move || {
            let txc = tx.send(Arc::new(42));
            assert_eq!(txc, 12);
            let txc = tx.send(Arc::new(69));
            assert_eq!(txc, 12);
            std::thread::sleep(Duration::from_millis(500));
            let txc = tx.send(Arc::new(80085));
            assert_eq!(txc, 12);
        });

        let mut rvs = Vec::new();

        for rv in rx {
            rvs.push(std::thread::spawn(move || {
                assert_eq!(*rv.recv_one().unwrap(), 42);
                assert_eq!(*rv.recv_one().unwrap(), 69);
                assert_eq!(rv.try_recv_one(), TryRecvResult::WouldBlock);
                assert_eq!(*rv.recv_one().unwrap(), 80085);
            }));
        }

        for thread in rvs {
            assert!(thread.join().is_ok());
        }
    }

    #[test]
    fn test_million_messages_threaded() {
        const THREAD_COUNT: usize = 16;
        const MESSAGE_COUNT: usize = 1_000_000;
        let mut tx = BroadcastChannel::<usize>::new();

        let mut rx = Vec::new();

        for _ in 0..THREAD_COUNT {
            rx.push(tx.new_receiver());
        }

        let mut rvs = Vec::new();

        for rv in rx {
            rvs.push(std::thread::spawn(move || {
                for i in 0..MESSAGE_COUNT {
                    assert_eq!(*rv.recv_one().unwrap(), i)
                }
            }))
        }

        assert!(
            std::thread::spawn(move || {
                for i in 0..MESSAGE_COUNT {
                    assert_eq!(tx.send(Arc::new(i)), THREAD_COUNT);
                }
            })
            .join()
            .is_ok()
        );

        for thread in rvs {
            assert!(thread.join().is_ok())
        }
    }

    #[test]
    fn test_million_messages_threaded_many() {
        const THREAD_COUNT: usize = 16;
        const MESSAGE_COUNT: usize = 1_000_000;
        let mut tx = BroadcastChannel::<usize>::new();

        let mut messages = Vec::with_capacity(MESSAGE_COUNT);

        for i in 0..MESSAGE_COUNT {
            messages.push(Arc::new(i))
        }

        let mut rx = Vec::new();

        for _ in 0..THREAD_COUNT {
            rx.push(tx.new_receiver());
        }

        let mut rvs = Vec::new();

        for rv in rx {
            rvs.push(std::thread::spawn(move || {
                for i in 0..MESSAGE_COUNT {
                    assert_eq!(*rv.recv_one().unwrap(), i)
                }
            }))
        }

        assert!(
            std::thread::spawn(move || {
                assert_eq!(tx.send_many(messages), THREAD_COUNT);
            })
            .join()
            .is_ok()
        );

        for thread in rvs {
            assert!(thread.join().is_ok())
        }
    }
}
