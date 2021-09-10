use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Listing 16-1: Creating a new thread to print one thing while the main thread prints something else
fn main() {
    // Listing 16-2: Saving a JoinHandle from thread::spawn to guarantee the thread is run to completion
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    // Listing 16-10: Sending multiple messages and pausing between each
    let thread1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    thread1.join().unwrap();
    println!("--------------------");

    // Listing 16-11: Sending multiple messages from multiple producers
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("--------------------");
}

#[cfg(test)]
mod Test_Concurrency {
    use std::rc::Rc;
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // Listing 16-3: Attempting to use a vector created by the main thread in another thread
    // Listing 16-4: A thread with a closure that attempts to capture a reference to v from a main thread that drops v
    // Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses
    #[test]
    fn test16_3() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        // drop(v);

        handle.join().unwrap();
    }

    // Listing 16-6, 16-7, 16-8: Moving tx to a spawned thread and sending “hi”
    #[test]
    fn test16_6() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = "hi".to_string();
            tx.send(val).unwrap();
            // println!("val is {}", val); // val has move to send()
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    //Listing 16-12: Exploring the API of Mutex<T> in a single-threaded context for simplicity
    #[test]
    fn test16_12() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    // Listing 16-13: Ten threads each increment a counter guarded by a Mutex<T>
    #[test]
    fn test16_13() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let t = thread::spawn(move || {
                let mut cc = counter.lock().unwrap();
                *cc += 1;
            });

            handles.push(t);
        }

        for t in handles {
            t.join().unwrap();
        }

        println!("counter = {}", *counter.lock().unwrap());
    }

    // This method will cause deadlock
    #[test]
    fn test_deadlock() {
        let a = Arc::new(Mutex::new(1));
        let b = Arc::new(Mutex::new(1));
        let a1 = Arc::clone(&a);
        let b1 = Arc::clone(&b);
        let thread_a = thread::spawn(move || {
            let mut aa = a1.lock().unwrap();
            println!("Thread A lock a.");
            thread::sleep(Duration::from_secs(1));
            let mut bb = b1.lock().unwrap();
            println!("Thread A lock b.");
        });
        let a2 = Arc::clone(&a);
        let b2 = Arc::clone(&b);
        let thread_b = thread::spawn(move || {
            let mut bb = b2.lock().unwrap();
            println!("Thread B lock a.");
            thread::sleep(Duration::from_secs(1));
            let mut aa = a2.lock().unwrap();
            println!("Thread B lock b.");
        });
        thread_a.join().unwrap();
        thread_b.join().unwrap();
    }
}
