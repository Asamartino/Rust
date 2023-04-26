# Summary of Chapter 3

**Rust**:
- **statically typed language**: variable types must be known at compile time
- **Expression based language** -> everything in Rust is an expression.
	- expression: evaluate to a resulting value
		- calling a function, calling a macro, the block {…}, math expression, a value e.g. 5, if statement, etc.
		- do not include ending semicolons -> adding one turns it into a statement.
	- statement: perform some action and do not return a value.
		- creating and assigning a value to a variable with let, function definition, etc.
	- **snake case**: for function and variable names, i.e. lowercase letter and underscores separated words.
	- by default, variables are immutable -> the rust compiler guarantees it :)

**Constant**: always annotate type:
```rust
const MAX_points: u32 = 100_000;
```

**Shadowing**: by using _let_ and same variable name -> perform a few transformations or even change the type -> variable will be immutable after those transformations are completed.
```rust
let x = 5.
let x = x + 1;

let spaces = "   ";
let spaces = spaces.len();
```
**Char**: specified with single quote (string uses double quote). Represents Unicode Scalar Value -> more than just ASCII (emoji, hiragana, etc..).

**Compound type**: to group multiple values into one type.
- Tuple: general way of grouping together different values into one compound type:
```rust
let tup: (i32,f64,u8) = (500,6.4,1);

let (x, y, z) = tup;  // destructing
let x = tup.0         //using its index
```

- Array: fixed length  (can’t grow or shrink once declared) + all variables must have same type. Data allocated on the stack.
```rust
let a = [1,2,3,4,5];

let first = a[0];
let second = a[1];
```

**Stack**: stores value in the order it gets and removes them in LIFO. **Works fast, data of known size**.

**Heap**: less organized. When put data on the heap, you request a certain amount of space. The operating system finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer. **Slow** (as need to follow a pointer), **data of unknown size** at compile time.

**Functions**:
- Rust doesn’t care where you define your function as long as it’s defined somewhere.
- _main()_: lines execute in the order in which they appear

**If expression**: 
- bool as condition (no automatic conversion)
- Multiple conditions with _else if_: 
	- only executes the block for the first true condition -> doesn’t even check the rest 
```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
//output: number is divisible by 3
```
- both arms _if else_ must return the same type

**Loop**:
- _loop_: executed forever or until you stop it with _break_
- _while_: concise combination of _loop_, _if_, _else_ and _break_
- _for_: **used for safety and conciseness (most commonly used)**
```rust
let a = [10, 20, 30, 40, 50];
for element in a.iter() {    // increased safety and ↘ bugs
    println!("the value is: {}", element);
}

for number in (1..4).rev() {
    println!("{}!", number);
}
println!("LIFTOFF!!!");
 ```

