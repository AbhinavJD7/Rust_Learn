use std::thread;
use std::time::Duration; //duration lib for time management
fn main(){                          
    let handle = thread::spawn(||{ //thread::spawn creates a new thread that runs concurrently with the main thread 
        for i in 1..10{             // || this is a closure it contains code to run in the new thread
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }     // join() blocks the main thread and waits for the spawned thread to complete
    });      // .unwrap() handles the Result type (panics if the thread had an error)
    //handle.join().unwrap(); //wait for the thread to finish then move forward

    for i in 1..5 { // this is the main thread the above was spawned thread
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(i));
    }
    handle.join().unwrap();
}

// Execution Flow:-
// Spawned thread starts printing (1-10) 
// Main thread waits with join()
// Once spawned thread completes, main thread prints (1-5)
// But now I have removed .join() from the spawned thread loop so now it will be randomly picking things from
// main thread and sometimes from spawned thread 