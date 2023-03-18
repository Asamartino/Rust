# Summary of chapter 6
**Enum**: use when you can enumerate all possible value. Each variant can have different types and amounts of associated data. Can also define methods on enum.
```rust
enum IpAddrKind {
      V4(u8,u8,u8),
      V6(String)
}
let four = IpAddrKind::V4;
```

**Rust does not have nulls**, -> Option<T>: encodes the concept of value being present or absent:
```rust
enum Option<T>{
    Some(T),
    None,
}
```
If use _None_ rather than _Some_ -> need to specify the type as compiler can’t infer it:
```rust
let absent_number: Option<i32> = None;
```

**_match_**: **exhaustive control flow operator** that compares a value against a series of patterns and then executes code based on the **1st pattern the value “fits”**.
- Match guard: extra condition on a match arm that further refines the arm’s pattern:
    ```rust
    match value {
        Some(x) if x > 5 => println!("Value is greater than 5"),
        Some(x) if x < 0 => println!("Value is negative"),
        Some(x) => println!("Value is {}", x),
        None => println!("No value"),
    }
    ```
- Underscore:
    - _ _ pattern_ will match any value. By putting after other arms _ will match all possible cases that aren’t specified before it. 
    - _s ignoring an unused variable.
    ```rust
    fn add_numbers(f: i32, _s: i32) -> i32 {
        f + 1
    }
    ```
- _if let_:  handle values that match 1 pattern while ignoring the rest. **Be careful of implicit coercion**
```rust
let some_u8_value = Some(ou8);
match some_u8_value{
    Some(3) => println!("three"),
    _ => (),
}
// is equivalent to
if let Some(3) =  some_u8_value {
    println!("three");
}
```
-> trade-off as, loose exhaustive checking that match enforces. Can also include an else

