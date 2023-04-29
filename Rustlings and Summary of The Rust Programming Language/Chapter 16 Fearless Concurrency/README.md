# Summary of chapter 16

The Rust team discovered that ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems. By using them may concurrency errors become compile-time errors  fearless concurrency.

**Threads**: provide a way to split a program into multiple independent tasks that can be executed concurrently, potentially speeding up the program execution but also adds complexity.
Rust standard library only provides an implementation of 1:1 threading.
- To create a new thread:
    ```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }); // will be stopped when the main thread ends even if it has not finished running

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    ```
    ```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
        ... // same code as above
        handle.join().unwrap();
    }
    ```

-	The return type of thread::spawn is JoinHandle. By calling join, will wait for its thread to finish (blocks the thread currently running until the thread represented by the handle terminates)  depending where join is call affects whether or not your threads run concurrently.
-	move closure: force the closure to take ownership of the values it uses. Allows to use data from one thread in another thread.
use std::thread;
```rust
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
```
