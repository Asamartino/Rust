# Summary of chapter 4
All programs manage the way they use a computer’s memory while running. In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks at compile time  managing heap data is why ownership exists.

Ownership rules:
-	Each value in Rust has a variable called its owner.
-	There can only be one owner at a time.
-	When the owner goes out of scope, the value will be dropped. Rust calls drop automatically at }


String: (stored on the heap) made up of three parts: a pointer, a length, and a capacity
 
let s2 = s1;
-	for stack data:
o	copy the data
-	for heap data:
o	move the data: rust invalidates the first variable  prevent double free error. (to create a deep copy use .clone())
 

 Passing a variable to a function will move or copy the variable.

& = reference: (=pointer) to avoid taking ownership. As you borrow the data, can’t modify it. 
 

Rule of references:
-	At any given time, you can have either but not both: 
o	one mutable reference 
o	or any number of immutable references
-	References must always be valid.

Slice: a reference (= pointer) to a contiguous sequence of elements in a collection (string, array, etc.)

String slice (str): reference to part of a String
let s = String::from("hello world");
let hello = &s[0..5];   // from start to 5 (not included)
let hello = &s[..5];    
let world = &s[6..=10]; // from start to 10 (included)
let world = &s[6..];
let whole_string = &s[..]; // or simply &s

 

Taking &str instead of &String makes our function more general and useful. As can pass string slice directly or a slice of a String:
fn first_word(s: &str) -> str{…}








