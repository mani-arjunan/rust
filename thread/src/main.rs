use std::{sync::{Arc, Mutex, mpsc}, thread::{self, spawn}};

// simple thread
// fn main() {
//     thread::spawn(|| {
//         for i in 0..5 {
//             println!("Inside spawned thread{}", i);
//         }
//     });
//
//     // this will run first or second, depending on the system, but as soon as the
//     // code in the main thread completes this functions popped off,
//     // any remaining execution pending on any spawned thread will be gone.
//     for i in 0..5 {
//         println!("Inside main thread{}", i);
//     }
// }

// spawned thread with wait time
// fn main() {
//     let spawned_thread = thread::spawn(|| {
//         for i in 0..5 {
//             println!("Inside spawned thread{}", i);
//         }
//     });
//
//     // this will run first or second, depending on the system, but as soon as the
//     // code in the main thread completes this functions popped off,
//     // any remaining execution pending on any spawned thread will be gone.
//     for i in 0..5 {
//         println!("Inside main thread{}", i);
//     }
//
//     // this is kinda sync code, where it makes the program to wait untill the spawned_thread
//     // completes its task in its thread.
//     spawned_thread.join().unwrap();
// }

// spawned thread with wait time
// fn main() {
//     let spawned_thread = thread::spawn(|| {
//         for i in 0..5 {
//             println!("Inside spawned thread{}", i);
//         }
//     });
//
//     // this will run first or second, depending on the system, but as soon as the
//     // code in the main thread completes this functions popped off,
//     // any remaining execution pending on any spawned thread will be gone.
//     for i in 0..5 {
//         println!("Inside main thread{}", i);
//     }
//
//     // this is kinda sync code, where it makes the program to wait untill the spawned_thread
//     // completes its task in its thread.
//     spawned_thread.join().unwrap();
// }

// message passing using channels
// fn main() {
//     let (sender, receiver) = mpsc::channel();
//     // move is needed in the below closure function, coz sender variable is being used from main
//     // thread inside the spawned thread
//     let spawned_thread = thread::spawn(move || {
//         let name = String::from("Manikandan");
//         // send method returns Result<(), err>, coz there may be scenario where receiver from the
//         // same channel could have been closed before send
//         sender.send(name).unwrap();
//         // println!("{}", name); won't work coz we are moving the ownership to main/receiver thread
//     });
//
//     if let Ok(name) = receiver.recv() {
//         println!("{}", name);
//     }
//
//     spawned_thread.join().unwrap();
// }

// Concurrency using Mutex
fn main () {
    // creates a new data 0 under the heap
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Arc allows multiple threads to share ownership of the same data.
        // Arc::clone increments the reference count, NOTE: does NOT duplicate the value.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    println!("{}", *counter.lock().unwrap());
}






























