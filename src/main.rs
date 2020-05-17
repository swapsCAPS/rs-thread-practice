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

mod lib;

use std::thread;
use std::time::Duration;

fn main() {
    let mut yolo_thread1 = lib::YoloThread::new();
    let mut yolo_thread2 = lib::YoloThread::new();

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
