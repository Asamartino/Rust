# Summary of chapter 11
**Testing**: to ensures that the code functions the way we intend -> complex skill to master

_assert!_(Boolean)
- if true: does nothing and test passes
- if false: _panic!_

_assert_eq!_  and _assert_ne!_ compare two arguments for equality or inequality. If assertion fails prints the two values -> makes it easier to see why the test failed. Value being compared must implement _PartialEq_ and _Debug_ traits.
-_ assert_ne!_: when know what the value definitely won’t be x (f.i. function that change code depending on the day of the week)

Can also add custom message to be printed with failures message as optional arguments to _assert!_, _assert_eq!_ and _assert_ne!_:
```rust
let result = greeting("Carol")
assert!(
    result.contains("Carol"),
    "Greeting did not containe name, value was '{}'", result
)
```
