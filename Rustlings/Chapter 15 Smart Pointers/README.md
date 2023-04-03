# Summary of chapter 15
**Smart pointers**: data structures that act like a pointer but also have additional metadata and capabilities (f.i. _String_ and _Vec\<T\>_). In many cases, they own the data they point to. **They all implement de _Deref_ and _Drop_ traits**.

Deref trait: allows you to customize the behavior of the dereference operator *. Behind the scene: *x = *(x.deref())
Deref coercion: converts a reference to a type that implements Deref into a reference to a type that Deref can convert the original type into. Happens automatically f.i. 
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //thanks to deref coercion don't have to write hello(&(*m)[..]);
}
No runtime penalty for taking advantage of deref coercion!


Drop trait: lets you customize what happens when a value is about to go out of scope (f.i. can run some code before value goes out of scope).
Rust automatically calls drop when our instance goes out of scope  to force a value being dropped before the end of its scope use:
std::mem::drop
...
drop(var)



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
