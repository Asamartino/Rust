# Summary of Chapter 3

**Rust**:
- **statically typed language**: compiler must know every variable's type before the program runs. So either you annotate it explicitly or the compiler infers it
```rust
let x: u32 = 5;     // you tell the compiler
let y = 5;           // compiler infers i32
let z = "hello";     // compiler infers &str
```
- **Expression based language** -> everything in Rust is an expression. 
	- Every expression produces something. **You can always put it on the right side of an = because it gives back a value**.
		- calling a function, calling a macro, the block {…}, math expression, a value e.g. 5, if statement, etc.
		```rust
		let x = 5;          // 5 is an expression, it produces → 5
		let x = 5 + 3;      // 5 + 3 is an expression, it produces → 8

		fn add(a: i32, b: i32) -> i32 {
			a + b
		}
		let x = add(2, 3);  // add(2, 3) is an expression, it produces → 5

		let x = {
			let a = 10;
			let b = 20;
			a + b          // no semicolon! last expression is the value,
		};
		// the whole block is an expression, it produces → 30
		```
	- statement: perform some action and do not return a value.
		- creating and assigning a value to a variable with let, function definition, etc.
		```rust
		let x = 5;             // stores 5 in x, but "let x = 5" itself produces nothing
		let y = (let x = 5);   // ❌ error — nothing to assign to y (proof that it produces nothing)

		fn greet() {            // defines a function, produces nothing
			println!("hi");
		}		
		```



	- **snake case**: for function and variable names, i.e. lowercase letter and underscores separated words.
	- by default, **variables are immutable** -> the rust compiler guarantees it :)

**Constant**: you must explicitly specify the type:
```rust
const MAX_points: u32 = 100_000;
```

**Shadowing**:  you can declare a new variable with the same name as a previous one. This new declaration *shadows* (hides) the old one. Can perform a few transformations or even change the type -> variable will be immutable after those transformations are completed.
```rust
let x = 5.
let x = x + 1;                  // x is now 6, old x is gone

let spaces = "   ";            // &str
let spaces = spaces.len();     // now it's usize —> totally different type
```
**Char**: specified with single quote (string uses double quote). Rust's char is 4 bytes and represents a full Unicode scalar value. Unicode is basically a giant list where every character in the world gets a unique number (contains emoji, hiragana, etc..)

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
- condition must be a bool (no automatic conversion)
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
There are 3 types of loop in Rust:
- _loop_: executed forever or until you stop it with _break_
- _while_: concise combination of _loop_, _if_, _else_ and _break_
- _for_: **used for safety and conciseness (most commonly used)**
```rust
let a = [10, 20, 30, 40, 50];
for element in a.iter() {    // increased safety and ↘ bugs
 println!("the value is: {}", element);
}

// Output:
// the value is: 10
// the value is: 20
// the value is: 30
// the value is: 40
// the value is: 50

for number in (1..4).rev() {
    println!("{}!", number);
}
println!("LIFTOFF!!!");

// Output:
// 3!
// 2!
// 1!
// LIFTOFF!!!


 ```

