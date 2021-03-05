use std::thread;
use std::time::Duration;
fn thread_or_main_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("THREAD OUTPUT: {}", i);
            thread::sleep(Duration::from_millis(1));   // sleep and allow execution of other threads
        }
    });
    for i in 1..5 {
        println!("MAIN OUTPUT: {}", i);
        thread::sleep(Duration::from_millis(1));       // sleep and allow execution of other threads
    }
}
fn main(){
thread_or_main_thread();
println!("Function execution finished ...");

thread::sleep(Duration::from_millis(100));             // wait for threads to finish (function does not wait)
}
