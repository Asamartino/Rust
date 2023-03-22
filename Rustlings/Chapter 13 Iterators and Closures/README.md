# Summary of chapter 13
**Closures**: **anonymous functions** that you can save in a variable or pass as arguments to other functions. Unlike functions, **can capture their environment** and access variables from the scope in which they’re defined. 
```rust
let expensive_closure = |num| {
    println!("claculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}
// expensive_closure contains the definition of an anonymous function // not its resulting value
// to call: expensive_closure(10)
```

- Don’t require you to annotate the types of the parameters or the return value like functions do (can add type annotations ↗ explicitness and clarity) -> type will be inferred -> can’t use different types (f.i. can’t call using an u32 as input and another time a String) -> generic
    ```rust
    fn add_one_v1    (x:u32) -> u32 {x+1}; // function
    let add_one_v2 = |x:u32| -> u32 {x+1}; // closure
    let add_one_v3 = |x|            {x+1}; // closure
    let add_one_v4 = |x|             x+1 ; // closure
    ```
- Capture values from their environment in three ways: _FnOnce_, _FnMut_, _Fn_
    - **_move_**: to take ownership of the values it uses
        ```rust
        let equal_to_x = move |z| z==x
        ```
\
**Iterators**: allows you to perform some tasks on a sequence of items. 
- Depending on the use case: _iter()_, _into_iter()_, _iter_mut()_.
- Handles the logic of iterating over each item and determining when sequence has finished. 
- **lazy**: no effect until call methods that consumes it.
- **consuming adaptors**: methods that uses up the iterator (f.i. _next()_)
- **iterator adaptors**: change iterators into different kinds of iterators -> as lazy need to call a consuming adaptor to get the result (f.i. _collect()_, _sum()_)
     ```rust
     v1.iter().map(|x| x + 1); // doesn't work
     let v2 : Vec<_> = v1.iter().map(|x| x + 1).collect();
     ```
- all iterators implement trait _Iterator_ -> requires implementors to define _next()_ -> can create your own iterator :) by impl _Iterator_ and defining _next()_ -> can use any Iterator trait method’s default implementations as defined in the standard library.
    ```rust
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    // as next() is defined can now use any Iterator trait method’s default implementation 
    let sum:u32 = Counter::new().zip(Counter::new().skip(1))
                                .map(|a,b| a*b)
                                .filter(|x| x%3 == 0)
                                .sum(); 
    ```

**The implementation of closures and iterators are such that runtime performance is not affected.**


