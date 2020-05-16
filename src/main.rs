use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock };

fn spawn_thread (a_ref: &Arc<RwLock<u8>>) {
    fn writer (a_ref: Arc<RwLock<u8>>) {
        let mut go_back = false;
        loop {
            // NOTE that the lock is held until the end of the scope
            // So if we sleep at the end. We will hold a write lock so no one can read for that
            // duration.
            // We can use a drop() to release the lock.

            let mut w = a_ref.write().unwrap();


            if *w >= 255 {
                go_back = true;
            }

            if *w == 0 {
                go_back = false;
            }

            if go_back {
                *w -= 1;
            } else {
                *w += 1;
            }

            // NOTE drop() does nothing speacial. The function just takes ownership and thus the
            // memory is freed after its scope ends.
            drop(w);
            thread::sleep(Duration::from_millis(1));
        }

    }

    let clone = Arc::clone(&a_ref);

    thread::spawn(move || {
        writer(clone)
    });
}

fn main() {
    let fr = RwLock::new(0);
    let fl = RwLock::new(0);

    let arc_fr = Arc::new(fr);
    let arc_fl = Arc::new(fl);

    // NOTE See note at spawn_thread return value.
    spawn_thread(&arc_fr);
    spawn_thread(&arc_fl);

    loop {
        let fr = arc_fr.read().unwrap();
        let fl = arc_fl.read().unwrap();

        // NOTE
        // We could use this approach to build some sort of movement engine
        // Setting servo positions with the read values.
        // The threads will be in control of movement speed, positions, etc.
        // The main loop will just "set" the PWM signals for all servos continuously

        println!("fr {}", fr);
        println!("fl {}", fl);
        thread::sleep(Duration::from_millis(10));
    }
}
