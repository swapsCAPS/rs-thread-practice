// TODO
// - [-] I'd like to send a signal to a thread so it stops its loop. TODO needs nicer approach
// - [✓] It should have a start and stop function.
// - [✓] Probably achievable by creating a class / impl-struct
//
//   NOTE
//     it appears that it's quite difficult to have a struct "wrap" a thread handle...
//     I'm getting lifetime issues and I have not found an elegant notation yet.
//     This post has a comment detailing why this cannot work:
//     https://stackoverflow.com/questions/42043823/design-help-threading-within-a-struct
//     "The signature of ::spawn takes a closure with a 'static lifetime"
//     I'm trying to pass in &self which then of course also should be 'static.
//     https://www.reddit.com/r/rust/comments/7iuzy8/thread_living_as_long_as_a_struct/
//     The first post lists an alternative, but I'm not a fan. I also want to mutate values of self
//     withing the thread func, so I'm not sure it would even work.
//     This post seems to have a nice solution!
//     https://stackoverflow.com/questions/54058000/how-to-mutate-self-within-a-thread
//
//     sentry also has a nice post explaining the "inner" pattern:
//     https://blog.sentry.io/2018/04/05/you-cant-rust-that
//
// TODO
// - [ ] I'd like to be able to control the rate of increase
// - [ ] I'd like to be able to control the acceleration of increase
//
// TODO
// - [✓] I'd like a way to structure a lot of threads.
// - [✓] I'd like to be able to reference to them by name

// NOTE
// We could use this approach to build some sort of movement engine
// Setting servo positions with the read values.
// The threads will be in control of movement speed, positions, etc.
// The main loop will just "set" the PWM signals for all servos continuously
// Is it overkill to use threads for that? Probably. One could achieve the same with one loop at
// the beginning of the loop you could check for input and then update values using elapsed time
// and the given input. This way you could do acceleration, speed control etc.
// I guess this is more an excersize in threading than anything else. Hey! It's called
// rs-thread-practice right :D

use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock, Mutex };

type ArcMutex<T> = Arc<Mutex<T>>;

struct YoloThreadInner {
    alive: bool,
    go_back: bool,
}

struct YoloThread {
    inner: ArcMutex<YoloThreadInner>,
    value: Arc<RwLock<u8>>,
    handle: Option<thread::JoinHandle<()>>,
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
                thread::sleep(Duration::from_millis(10));
            }
        }));
    }
}

fn main() {
    let mut yolo_thread1 = YoloThread::new();
    let mut yolo_thread2 = YoloThread::new();

    yolo_thread1.init();
    yolo_thread2.init();

    yolo_thread1.start();
    yolo_thread2.start();

    loop {
        let val1 = yolo_thread1.value.read().unwrap();
        let val2 = yolo_thread2.value.read().unwrap();
        println!("vals {}, {}", val1, val2);

        if *yolo_thread2.value.read().unwrap() == 255 {
            yolo_thread2.stop();
        }

        thread::sleep(Duration::from_millis(1));
    }
}
