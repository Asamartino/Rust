# Summary of chapter 15
**Smart pointers**: data structures that act like a pointer but also have additional metadata and capabilities (f.i. _String_ and _Vec\<T\>_). In many cases, they own the data they point to. **They all implement de _Deref_ and _Drop_ traits**.

**Deref trait**: allows you to customize the behavior of the dereference operator *. Behind the scene: 
```rust 
*x = *(x.deref()) 
```
- **Deref coercion**: converts a reference to a type that implements _Deref_ into a reference to a type that _Deref_ can convert the original type into. Happens automatically f.i.:
    ```rust
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    fn main() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        //thanks to deref coercion don't have to write hello(&(*m)[..]);
    }
    ```
**No runtime penalty for taking advantage of deref coercion!**


**Drop trait**: lets you customize what happens when a value is about to go out of scope (f.i. run some code before value goes out of scope).
Rust automatically calls drop when our instance goes out of scope -> to force a value being dropped before the end of its scope use:
```rust
std::mem::drop
...
drop(var)
```


Most common smart pointers in the standard library:

**_Box\<T\>_**: pointer stored on the stack that points to data stored on the heap. Most common use case:
-	When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size.\
\
  At compile time Rust needs to know how much space a type takes up -> can’t use recursive types as their size can’t be known at compile time f.i.: Cons list: 
    ```rust
    enum List {    // error[E0072]: recursive type `List` has infinite size
        Cons(i32, List),
        Nil,
    }
    fn main() {
        let list = Cons(1, Cons(2, Cons(3, Nil)));
    }
    ```
    Similar to:\
    ![image](https://user-images.githubusercontent.com/61462365/229295790-50f97a8d-c2cf-4770-8e67-7c18c2f0712d.png)

    **A pointer’s size doesn’t change based on the amount of data it’s pointing to** -> by storing the value inside _Box\<T\>_ Rust will know how much space to allocate:

    ```rust
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    fn main() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
    ```
-	When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
-	When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

**Reference counting**: _Rc\<T\>_ enables an immutable value to have multiple owners. Keeps track of the number of references to a value to determine whether or not the value is still in use (only if references = 0 -> value can be cleaned up). 
- Use when want to allocate some data on the heap for multiple parts of your program to read and can’t determine at compile time which part will finish using the data last. **Only for single-threaded scenario**.
    ```rust
    use std::rc::Rc;
 
    fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // Rc::strong_count(&a) = 1
    let b = Cons(3, Rc::clone(&a)); // Rc::strong_count(&a) = 2
    let c = Cons(4, Rc::clone(&a)); // Rc::strong_count(&a) = 3
    }
    ```
- _Rc::clone_: doesn’t make a deep copy of all the data only ↗ _strong_count_ by 1 (doesn’t take much time). Rust automatically ↘ _strong_count_ by 1 when a value goes out of scope.
- _Rc::downgrade_: ↗ _weak_count_ by 1. _Weak_count_ does not need to be 0 for the _Rc\<T\>_ instance to be cleaned up. As the value might have been dropped -> check if value exist -> _upgrade_

**_RefCell\<T\>_**: enforces at runtime the rule of reference (see chapter 4). If you break these rules with _RefCell\<T\>_ -> your program will panic and exit (with reference and _Box\<T\>_ -> compilation error). Only for single-threaded scenarios. For mutable and immutable values.
- Useful when you’re sure your code follows the borrowing rules, but the compiler is unable to understand and guarantee that. 
- Interior mutability: can mutate the value inside it even when declared as  immutable.
    - Useful to mutate a value with its methods but appear immutable to other code.
- _borrow_ and _borrow_mut_ part of the safe API that belongs to _RefCell\<T\>_. Like borrowing rules can have many immutable borrows or one mutable borrow at any point in time.
- By having an _Rc\<T\>_ holding a _RefCell\<T\>_ -> can get a value that can have multiple owners and that you can mutate.


**You can’t rely on Rust to prevent memory leak**
