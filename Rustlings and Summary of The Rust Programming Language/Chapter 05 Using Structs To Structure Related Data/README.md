# Summary of chapter 5 
**Struct**: custom type that holds multiple values (≈ object):
  ```rust
struct User{
    username: String,// rather than &str ->instance owns its data and data is valid as long as entire struct lives
    email: String,
    age: u64,
    active: bool,
}
  ```
- Entire instance must be mutable in order to change one field.
- Syntactic sugar:
    - _Field init shorthand syntax_: When the variable name matches the field name, you don't have to write it twice:
      ```rust
      fn build_user(email: String, username: String) -> User{
          User{
              email,             // do not need to write : email : email, same with username
              username,
              active: true,
          }
      }
      ```
    - _Struct update syntax_: When you want to create a new struct but only change a few fields
      ```rust
        let user2 = User{
            email: String::from("another@eample.com"),
            username: String::from("anotherusername567"),
            ..user1             // ..user1 fills in the rest
        };
      ```
- Structs don't implement Display by default, so you can't use {} with println!. Instead, add #[derive(Debug)] and use {:?} or {:#?}.
    ```rust
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
    }

    let user = User { name: String::from("Alice"), age: 30 };

    println!("{}", user);    // ERROR — no Display
    println!("{:?}", user);  // User { name: "Alice", age: 30 }
    println!("{:#?}", user); // same but pretty-printed (multi-line)
    ```


**Tuple struct**:  has fields but no names
```rust
// Regular tuple — no name at all
let color = (0, 0, 0);

// Regular struct — has named fields
struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

// Tuple struct — has fields but NO names, just positions
struct Color (i32,i32,i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
let origin = Point(0,0,0); 
// black ≠ origin as they're different tuple structs
```
**Unit struct**: don’t have any fields -> seems useless on its own, but becomes useful when you attach behavior to it with traits (see chapter 10). Like () (the unit type), it holds nothing, but unlike () it has a unique name so the compiler can distinguish between different unit structs.
```rust
struct MyUnitStruct;
```

**Methods**: function defined within the context of a struct. **First parameter is always _self_**, Can take ownership of self, borrow self immutably or borrow self mutable just as they can with any other parameter.
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
let rect1_area = rect1.area();
```
**Associated functions**: don’t take _self_ as parameter, often used for constructors. Use `::` to call, e.g. `String::from`
```rust
impl Rectangle{
    fn square (size:u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}
let sq = Rectangle::square(3);
```
Methods act on an instance, while associated functions act on the type.




