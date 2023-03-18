# Summary of chapter 4
All programs **manage** the way they use a **computer’s memory** while running. In Rust, memory is managed through a **system of ownership** with a **set of rules** that the compiler checks at **compile time** -> managing heap data is why ownership exists.

**Ownership rules**:
-	Each value in Rust has a variable called its owner.
-	There can only be one owner at a time.
-	When the owner goes out of scope, the value will be dropped. Rust calls drop automatically at }


**String**: (stored on the heap) made up of three parts: a pointer, a length, and a capacity.

<img align="center" src="https://user-images.githubusercontent.com/61462365/226119542-65c6bc05-cd9f-4d32-bcb6-77f9342b70b9.png">
 
```rust
let s2 = s1;
```
-	for **stack data**:
    - the data is copied
-	for **heap data**:
    - the data is **moved** -> Rust invalidates the first variable -> prevent _double free error_ (to create a deep copy use _.clone()_).
    
![image](https://user-images.githubusercontent.com/61462365/226119790-32d17f3a-71e9-420c-9ef5-737e557cc0ed.png)


-> Passing a variable to a function will move or copy the variable.

**& = reference**: (=pointer) to avoid taking ownership. As you **borrow** the data, can’t modify it. 
![image](https://user-images.githubusercontent.com/61462365/226119960-625fe82b-ed8b-4f20-8e7a-80b37392642d.png)

**Rule of references**:
-	At any given time, you can have either but not both: 
    -	one mutable reference 
    -	or any number of immutable references
-	References must always be valid.

**Slice**: **reference (= pointer) to a contiguous sequence of elements in a collection** (string, array, etc.)

**String slice (str)**: reference to part of a String
```rust
let s = String::from("hello world");
let hello = &s[0..5];   // from 0 to 5 (not included)
let hello = &s[..5];    
let world = &s[6..=10]; // from 6 to 10 (included)
let world = &s[6..];
let whole_string = &s[..]; // or simply &s
```
![image](https://user-images.githubusercontent.com/61462365/226120461-e1e1baa6-221a-4180-94d2-0c690898f912.png)

**Taking &str instead of &String makes our function more general and useful**. As can pass string slice directly or a slice of a _String_:
```rust
fn first_word(s: &str) -> str{…}
```








