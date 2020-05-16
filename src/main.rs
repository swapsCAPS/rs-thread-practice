use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock, Mutex};

fn spawn_thread (a_ref: Arc<RwLock<u8>>) -> Arc<RwLock<u8>> {
    fn writer (a_ref: Arc<RwLock<u8>>) {
        loop {
            // NOTE that the lock is held until the end of the scope
            // So if we sleep at the end. We will hold a write lock so no one can read for that
            // duration.
            // We can use a drop() to release the lock.

            let mut w = a_ref.write().unwrap();
            *w += 1;
            drop(w);
            thread::sleep(Duration::from_millis(100));
        }

    }

    let clone = Arc::clone(&a_ref);

    thread::spawn(move || {
        writer(clone)
    });

    return a_ref
}

fn main() {
    let fr = RwLock::new(0);
    let fl = RwLock::new(0);

    let mut arc_fr = Arc::new(fr);
    let mut arc_fl = Arc::new(fl);

    arc_fr = spawn_thread(arc_fr);
    arc_fl = spawn_thread(arc_fl);

    loop {
        let fr = arc_fr.read().unwrap();
        let fl = arc_fl.read().unwrap();

        if *fr >= 255 || *fl >= 255 {
            std::process::exit(0)
        }

        println!("fr {}", fr);
        println!("fl {}", fl);
        thread::sleep(Duration::from_millis(10));
    }
}
