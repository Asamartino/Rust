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

_#[should_panic]_: pass if code inside the function panics and fails if doesn’t -> imprecise as test can panic for a different reason.\
-> _#[should_panic(expected = “substring”]_: will pass if the expected value is a substring of the _panic!_ message -> depend on how precise you want to be.

**Tests run in parallel by default** -> make sure your tests don’t depend on each other or any shares state, including a shared environment:
- _cargo test -- --test-threads=1_: to not use any parallelism (slower but won’t interfere with each other).
- _cargo test –nocapture_: allow to see printed values for passing tests as well (normally will see only _println!_ if test fails)
- _cargo test subtest_name_: if want to run a particular test. 
    - To run multiple tests: specify part (substring) of a test name

_#[ignore]_: to exclude a test (f.i. if time consuming and want to ignore it during most runs).

