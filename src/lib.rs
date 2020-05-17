use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock, Mutex };

type ArcMutex<T> = Arc<Mutex<T>>;

struct YoloThreadInner {
    alive: bool,
    go_back: bool,
}

pub struct YoloThread {
    inner: ArcMutex<YoloThreadInner>,
    pub value: Arc<RwLock<u8>>,
    pub handle: Option<thread::JoinHandle<()>>,
}

impl YoloThread {
    pub fn new() -> Self {
        YoloThread {
            // NOTE inner will be used internally
            inner: Arc::new(Mutex::new(YoloThreadInner {
                alive: false,
                go_back: false,
            })),
            // NOTE value and handle are "public" values
            // If we would put value inside inner, we would have to lock the whole inner just to
            // read out one value. Might not be a big deal, but seems a bit ugly.
            value: Arc::new(RwLock::new(0)),
            handle: None
        }
    }

    pub fn stop(&self) {
        let mut i = self.inner.lock().unwrap();
        i.alive = false;
    }

    pub fn start(&self) {
        let mut i = self.inner.lock().unwrap();
        i.alive = true;
    }

    pub fn init(&mut self) {

        let inner = self.inner.clone();
        let value = self.value.clone();

        self.handle = Some( thread::spawn(move || {
            loop {
                let mut i = inner.lock().unwrap();
                if i.alive == false { break; }

                // NOTE that the lock is held until the end of the scope
                // So if we sleep at the end. We will hold a write lock so no one can read for that
                // duration.
                // We can use a drop() to release the lock.
                let mut w = value.write().unwrap();

                // TODO research why this deref is necessary here, but not for if i.alive at the
                // start of the loop... bool is treated differently?
                if *w >= 255 {
                    i.go_back = true;
                }

                if *w == 0 {
                    i.go_back = false;
                }

                if i.go_back {
                    *w -= 1;
                } else {
                    *w += 1;
                }

                // NOTE drop() does nothing speacial. The function just takes ownership and thus the
                // memory is freed after its scope ends.
                drop(i);
                drop(w);
                thread::sleep(Duration::from_millis(10));
            }
        }));
    }
}

