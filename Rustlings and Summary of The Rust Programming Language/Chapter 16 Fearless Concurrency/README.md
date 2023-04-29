# Summary of chapter 16

The Rust team discovered that ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems. By using them may concurrency errors become compile-time errors -> fearless concurrency.

**Threads**: provide a way to split a program into multiple independent tasks that can be executed concurrently, potentially speeding up the program execution but also adds complexity.\
Rust standard library only provides an implementation of 1:1 threading ->many concurrency solutions are implemented as crates -> search online for the current state of the art.
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

- The return type of _thread::spawn_ is _JoinHandle_. 
- By calling _join_, will wait for its thread to finish (blocks the thread currently running until the thread represented by the handle terminates) 
 -> depending where _join_ is call affects whether or not your threads run concurrently.
- _move_ closure: force the closure to take ownership of the values it uses. Allows to use data from one thread in another thread.
    ```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }
    ```
    
&nbsp;
&nbsp;

**Message passing**: threads communicate by sending each other messages containing data.
- Channel ≈ river of water. Has two halves:
  - Transmitter
  - Receiver
  - Closed: if either transmitter or receiver is dropped
    ```rust
    use std::sync::mpsc;  //multiple produce single producer
    use std::thread;
    
    fn main() {
        let (tx, rx) = mpsc::channel();  // tx: transmitter, rx: receiver

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap(); // tx.send() returns a Result<T,E> 
        });

        let received = rx.recv().unwrap(); // rx.recv() blocks main thread until a value is sent down the channel, return Result<T,E>
        // rx.try_recv(): also return Result<T,E> but doesn’t block thread -> useful if want to do other work while checking from time to time if received a message (by using a loop)
        println!("Got: {}", received);
    }
    ```
- To create multiple threads use _clone()_:
    ```rust
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    ```
    
&nbsp;
&nbsp;

**Mutex** (= mutual exclusion): allows only one thread to access some data at any given time. 
- To access it, a thread must acquire the mutex’s lock (keeps track of who currently has exclusive access to the data). 
- Two rules:
  - You must attempt to acquire the lock before using the data.
  - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
      ```rust
      use std::sync::Mutex;

      fn main() {
          let m = Mutex::new(5);

          {
              let mut num = m.lock().unwrap();
              *num = 6;
          }

          println!("m = {:?}", m); // m = 6
      }
      ```
- _Mutex\<T\>_ is a smart pointer. _lock()_ returns a smart pointer called _MutexGuard_.
  - Implements _Deref_ and _Drop_ (release the lock automatically when goes out of scope).
  - Comes with the risk of creating deadlocks
    
&nbsp;
&nbsp;

Extensible concurrency with _Sync_ and _Send_ Traits:
- _Send_ trait: indicates that ownership of the type implementing _Send_ can be transferred between threads.
- _Sync_ trait: indicates that it is safe for the type implementing _Sync_ to be referenced from multipele threads. In other words, any type _T_ is _Sync_ if _&T_ (is _Send_, -> the reference can be sent safely to another thread)

  **Types that are made up of _Send_ and _Sync_ traits are automatically also _Send_ and _Sync_, we don’t have to implement those traits manually** -> building new concurrent types not made up of _Send_ and _Sync_ parts requires careful thought to uphold the safety guarantees. 


