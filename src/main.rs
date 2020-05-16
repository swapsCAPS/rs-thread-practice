use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock };

struct Items {
    fr: RwLock<u8>,
    fl: RwLock<u8>,
}

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
            // thread::sleep(Duration::from_millis(1));
        }

    }

    let clone = Arc::clone(&a_ref);

    thread::spawn(move || {
        writer(clone)
    });
}

// TODO
// - I'd like to send a signal to a thread so it stops its loop.
// - It should have a start and stop function.
// - Probably achievable by creating a class / impl-struct
//   NOTE
//     it appears that it's quite difficult to have a struct "wrap" a thread handle...
//     I'm getting lifetime issues and I have not found an elegant notation yet.
//     https://stackoverflow.com/questions/42043823/design-help-threading-within-a-struct
//     https://www.reddit.com/r/rust/comments/7iuzy8/thread_living_as_long_as_a_struct/
//     The first post lists an alternative, but I'm not a fan. I also want to mutate values of self
//     withing the thread func, so I'm not sure it would even work.
//     This post seems to have a nice solution!
//     https://stackoverflow.com/questions/54058000/how-to-mutate-self-within-a-thread
//
// TODO
// - I'd like to be able to control the rate of increase
// - I'd like to be able to control the acceleration of increase
//
// TODO
// - I'd like a way to structure a lot of threads.
// - I'd like to be able to reference to them by name
//   So not a vec and then reference by index.
//   I want to be able to say <thread_name>.stop().
//   There is no way to forEach over a struct. So what is a nice approach?

fn main() {
    let items = Items {
        fr: RwLock::new(0),
        fl: RwLock::new(0),
    };

    let ar_items = Arc::new(&items);

    let arc_fr = Arc::new(items.fr);
    let arc_fl = Arc::new(items.fl);

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

        println!("fr {} fl {}", fr, fl);
        thread::sleep(Duration::from_millis(1));
    }
}
