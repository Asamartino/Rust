# Summary of chapter 18

Patterns come in two forms:

- Refutable: can fail to match for some possible value `if let Som(x) = value` (as value can be `None`); e.g.: `if let` and `while let`
- irrefutable: will match for any possible value, e.g. `if let x = 5`; e.g.: function parameters, `let` statements and `for` loops.

Match arms must use refutable pattern, except for the last arm, which should match any remaining values with an irrefutable pattern.

**Destructuring**:
```rust
let Point {x: a, y: b} = Point {x:0, y:7}; // create variable a=0 and b=7
let Point {x, y} = Point {x:0, y:7}; // create variable x=0 and y=7
let ((feet, inches), Point {x, y}) = ((3,10), Point {x:3, y: -10});
// create 4 values:  feet=3, inches=10, x=3, y=-10
```

- When the value we’re matching to our pattern contains a reference, we need to destructure the reference from the value (use `&` in the pattern):

  ```rust
  let points = vec![
      Point{x:0, y:0},
      Point{x:1, y:2},
  ];
  let sum_of_squares: i32 = points.iter().map(|&Point {x, y}| x*x + y*y).sum();
  ```
<br/>

**Ignoring Values in Pattern**:

- `_` will match any value but not bind to the value -> subtle difference btw `_` and using `_name`:

  ```rust
  let s = Some(String::from("Hello!"));
  if let Some(_s) = s {
      println!("found a string");
  }
  println!("{:?}", s); // won't work as s will be moved into _s
  /////////////////////
  if let Some(_) = s {
      println!("found a string");
  }
  println!("{:?}", s) // will compile as _ doesn't ever bind to the value -> s isn’t moved
  ```
- `_x` will tell Rust not to warn you about this unused variable
- `..` ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern. Will expand to as many values it needs to but must be unambiguous.
  ```rust
  let origin = Point {x:0, y:0, z:0};
  match origin {
      Point{x, ..} => println!("x is {}", x)
  }
  ```
  <br/>

**Creating references in patterns**:
When you match against a pattern, the variables introduced by the pattern are bound to a value. Rust’s ownership rules mean the value will be moved into the match or wherever you’re using the pattern (see above example). How to borrow a value ? -> use `ref` or `mut ref`

```rust
let robot_name = Some(String::from("Megaman"));
match robot_name {
    Some(ref name) => println!("found a name {}", name), // only takes a reference -> robot_name is not moved into name
    _ => (),
}
println!("robot_name is {:?}", robot_name);

// using mut ref
let mut robot_name = Some(String::from("Megaman"));
match robot_name {
Some(ref mut name) => *name = String::from("Robocop"),
_ => (),
}
println!("robot_name is {:?}", robot_name);
```

Can also use or operator in match guard:

```rust
  let x = 4;
  let y = false;
  match x {
      4 | 5 | 6 if y => println!("yes"), // equivalent to (4|5|6) if y=> ...
      _ => println!("no"),
  }
```
  <br/>
  
**Bindings**:
`@`: let us create a variable that holds a value and at the same time let’s you test it.

```rust
  enum Message {
      Hello { id: i32 },
  }

  let msg = Message::Hello { id: 5 };

  match msg {
      Message::Hello {
          id: id_variable @ 3..=7,
      } => println!("Found an id in range: {}", id_variable),
      Message::Hello { id: 10..=12 } => {
          println!("Found an id in another range")
      }
      Message::Hello { id } => println!("Found some other id: {}", id),
  }
```
