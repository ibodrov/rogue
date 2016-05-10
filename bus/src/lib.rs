#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct Inbox<T: Clone> {
        queue: Arc<Mutex<Vec<T>>>,
    }

    impl<T: Clone> Default for Inbox<T> {
        fn default() -> Self {
            Inbox {
                queue: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl<T: Clone> Inbox<T> {
        fn pop(&mut self) -> Option<T> {
            self.queue.lock().unwrap().pop()
        }
    }

    struct Dispatcher<T: Clone> {
        inboxes: Vec<Inbox<T>>,
    }

    impl<T: Clone> Default for Dispatcher<T> {
        fn default() -> Self {
            Dispatcher {
                inboxes: Vec::new(),
            }
        }
    }

    impl<T: Clone> Dispatcher<T> {
        fn publish(&self, v: T) {
            for i in &self.inboxes {
                i.queue.lock().unwrap().push(v.clone());
            }
        }

        fn subscribe(&mut self) -> Inbox<T> {
            let i = Inbox::default();
            self.inboxes.push(i.clone());
            i
        }
    }

    #[test]
    fn pubsub() {
        use std::thread;
        use std::time::Duration;

        let mut d = Dispatcher::default();

        let mut i1 = d.subscribe();
        let mut i2 = d.subscribe();

        let t1 = thread::spawn(move || {
            loop {
                if let Some(v) = i1.pop() {
                    println!("t1 got: {:?}", v);
                    break;
                } else {
                    println!("t1 is waiting");
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        let t2 = thread::spawn(move || {
            loop {
                if let Some(v) = i2.pop() {
                    println!("t2 got: {:?}", v);
                    break;
                } else {
                    println!("t2 is waiting");
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        d.publish("hi!");

        t1.join().unwrap();
        t2.join().unwrap();
    }
}
