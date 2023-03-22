# Summary of chapter 9
Rust errors:
- Recoverable: report the problem to the user. For reason that you can easily interpret and respond to. **Returning Result<T,E>  is a good default choice when you’re defining a function that might fail**
    - _unwrap()_: when have some logic that ensures the Result will have an Ok value.
        - if _Ok_ variant: return the value inside _Ok_
        - if Err variant: panic!
    - _expect()_: = _unwrap_ + let you choose the _panic!_ error message -> convey your intent and easier to find.
    - _unwrap_ and _expect_ are very handy when prototyping, before you’re ready to decide how to handle errors, **leave clear robust markers in your code for when you’re ready to make your program more robust.**
- Unrecoverable: (≈ bugs) use _panic!_ -> stop, print a failure message, unwind and clean up the stack, and then quit. Use _abort_ if need to make the resulting binary as small as possible (ends program without cleaning up). \
**Use panic! when your code could end up in a bad state**: when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values or missing values are passed to your code plus one or more of:
    - the bad state is not something that’s expected to happen occasionally
    - your code after this point needs to rely on not being in this bad state
    - there’s not a good way to encode this information in the types you use

**Backtrace**: list of all the functions that have been called to get to this point -> read until you see files you wrote.

**Propagating the error**: return the error to the calling code so it can decide what to do.
- **?: only in functions that have a return type of Result**:
    - if _Ok_ variant: return the value inside _Ok_ and program continues.
    - if _Err_ variant:  Error returned from the whole function as if used _return_ -> error value gets propagated to the calling code. The error will be converted in the type of error of the output function. 

**When your code performs operations on values, your code should verify the values are valid first and panic if they aren’t valid.**


**Trick**: use type annotation that you know is not a return type of the function -> when try to compile the code as types won’t matches, compiler will tell you what type is expected.




