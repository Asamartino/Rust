# Summary of chapter 5 
**Struct**: custom type that holds multiple values (≈ object):
```rust
struct User{
    username: String,// rather than &str ->instance owns its data! 
                     // + data is valid as long as entire struct is !
    email: String,
    age: u64,
    active: bool,
}
```
- Entire instance must be mutable in order to change a field.
- Syntactic sugar:
    - Field init shorthand syntax: 
    ```rust
      fn build_user(email: String, username: String) -> User{
          User{
              email,
              username,
              active: true,
          }
      }
      ```
    - Struct update syntax: **..**
    ```rust
        let user2 = User{
            email: String::from("another@eample.com"),
            username: String::from("anotherusername567"),
            ..user1
        }
     ```
	No implementation of _Display_ -> _#[derive(Debug)]_ and use _{:#}_ with _println!_

**Tuple struct**: to give the whole tuple a name -> different type.
```rust
struct Color (i32,i32,i32);
let black = Color(0,0,0);
let origin = Point(0,0,0); 
// black ≠ origin as they're different tuple structs
```
**Unit struct**: don’t have any fields, useful for generics, behave similarly to ()
```rust
struct MyUnitStruct;
```

**Methods**: function defined within the context of a struct. **First parameter is always self**, Can take ownership of self, borrow self immutably or borrow self mutable just as they can with any other parameter.
```rust
struct Rectangle{
    width: u32,    
    height: u32,
}
impl Rectangle{
    fn area(&self) -> u32{
        self.width* self.height
    }
}
let rect1 = Rectangle{width:30, height:50};
let area_rect1 = rect1.area();
```
**Associated functions**: don’t take self as parameter, often used for constructors. Use _::_ to call, e.g. _String::from_
```rust
impl Rectangle{
    fn square (size:u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}
let sq = Rectangle::square(3);
```




