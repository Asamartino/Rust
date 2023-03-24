# Summary of chapter 11
Testing: to ensures that the code functions the way we intend  complex skill to master

assert!(Boolean)
-	if true: does nothing and test passes
-	if false: panic!

assert_eq!  and assert_ne! compare two arguments for equality or inequality. If assertion fails prints the two values  makes it easier to see why the test failed. Value being compared must implement PartialEq and Debug traits.
-	assert_ne!: when know what the value definitely won’t be x (f.i. function that change code depending on the day of the week)

Can also add custom message to be printed with failures message as optional arguments to assert!, assert_eq! and assert_ne!
let result = greeting("Carol")
assert!(
    result.contains("Carol"),
    "Greeting did not containe name, value was '{}'", result
)
