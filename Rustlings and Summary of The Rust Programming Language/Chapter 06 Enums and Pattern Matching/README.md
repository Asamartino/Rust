# Summary of chapter 6
**Enum**: use when you can enumerate all possible value. Each variant can have different types and amounts of associated data. Can also define methods on enum.
```rust
enum IpAddrKind {
      V4(u8,u8,u8),
      V6(String)
}
let four = IpAddrKind::V4;
```

**Rust does not have nulls** -> _Option\<T\>_: encodes the concept of value being present or absent:
```rust
enum Option<T>{
    Some(T),
    None,
}
```
- if use _None_ rather than _Some_ -> need to specify the type as compiler can’t infer it:
    ```rust
    let x = None;                              // ✗ — None of what type? -> won't compile
    let absent_number: Option<i32> = None;     // ✓ — compiler knows this is Option<i32>
    ```

**_match_**: **exhaustive control flow operator** that compares a value against a series of patterns and then executes code based on the **1st pattern the value “fits”**. Exhaustive means you must cover every variant
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,            // if you remove Coin::Quarter, the compiler will complain -> prevents bugs from unhandled cases
    }
}
```

- Match guard: extra condition on a match arm that further refines the arm’s pattern:
    ```rust
    match value {
        Some(x) if x > 5 => println!("Value is greater than 5"),
        Some(x) if x < 0 => println!("Value is negative"),
        Some(x) => println!("Value is {}", x),
        None => println!("No value"),
    }
    ```
- can match multiple patterns using | (= or)
- `..` allow you to match an inclusive range of values or chars
    ```rust
     let x = 5;

    match x {
        1..=5 => println!("one through five"),
        6 | 7 => println!("six or seven"),
        _ => println!("something else"),
    }
    ```
- Underscore:
    - _ _pattern_ will match any value. By putting after other arms _ will match all possible cases that aren’t specified before it. 
    ```rust
    let num = 7;
    match num {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),  // covers all remaining values
    }
    ```
    - _s ignoring an unused variable.
    ```rust
    fn add_numbers(f: i32, _s: i32) -> i32 {
        f + 1
    }
    ```
- _if let_:  handle values that match 1 pattern while ignoring the rest. It's a syntactic sugar for a match where you only care about one pattern and want to ignore everything else. <!-- **Be careful of implicit coercion** why?-->
    ```rust
    let some_u8_value = Some(0u8);
    if let Some(3) =  some_u8_value {
        println!("three");
    }
    // is equivalent to
    match some_u8_value{
        Some(3) => println!("three"),
        _ => (),
    }
    ```

    - trade-off: **loose exhaustive checking**
    - can also include an _else_
    ```rust
      let mut count = 0;
      if let Coin::Quarter(state) = coin {
          println!("State quarter from {:?}", state);
      } else {
          count += 1;
      }
    ```
  It is also possible to mix and match `if let`, `else if` and `else if let` expressions e.g.:
  ```rust
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();
  
  if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
  }

  // prints: "Using purple as the background color"


  ```
      

