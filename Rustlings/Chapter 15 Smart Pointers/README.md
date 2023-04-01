# Summary of chapter 15
Smart pointers: data structures that act like a pointer but also have additional metadata and capabilities (f.i. String and Vec<T>). In many cases, they own the data they point to. They all implement de Deref and Drop traits.

Most common smart pointers in the standard library:

Box<T>: pointer stored on the stack that points to data stored on the heap. Most common use case:
-	When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size

At compile time Rust needs to know how much space a type takes up  can’t use recursive types as their size can’t be known at compile time f.i.: Cons list: 




