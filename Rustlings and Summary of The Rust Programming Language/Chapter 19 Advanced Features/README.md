# Summary of chapter 19
**Unsafe Rust**: doesn’t enforce memory safety guarantees (other Rust safety checks are still enabled, e.g. borrow checker, etc.). The _unsafe_ keyword only gives you access to four features: 
-	Dereference raw pointer
-	Call an unsafe function or method
-	Access or modify a mutable static variable
-	Implement an unsafe trait

-> **keep unsafe block small. Isolate unsafe code as much as possible** (e.g. enclose it within a safe abstraction and provide a safe API) -> will prevent your unsafe code from leaking. 
Using unsafe code isn’t wrong or to be frowned upon.

**Raw pointers**: _*const T_ and _*mut T_ (_*_ is part of the type name)
-	Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
-	Aren’t guaranteed to point to valid memory
-	Are allowed to be null
-	Don’t implement any automatic cleanup

-> give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.

-> be careful as can create data race
```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
// can create raw pointers (does no harm, only when try to access its value that you might end up with an invalid value) -> need unsafe keyword if want to dereference them
unsafe {
    println!("r1 is {}", *r1);
    println!("r2 is {}", *r2);
}
```
