# Summary of chapter 8
Collections (contains multiple values)  = **pointers with data stored on the heap** (≠ array or tuple) -> amount of data does not need to be known at compile time -> can grow or shrink as program run. Can be extended using _extend()_:
- _Vec\<T\>_
    - ``` vec!(..) = vec![..] ```
    - to access an element:
      ```rust
      let v = vec[1,2,3,4,5];
      let third: &i32 = &v[100];             // programs crash
      let third: Option<&i32> = v.get(100);  // returns None
      ```
    - to iterate:
      ```rust
        for i in &v{
            println!("{}", i);
        }
        for i in &mut v{
            *i += 50;     // * = dereference operator
        }
      ```
    - **capacity:** total number of elements it can hold without reallocating. 
    - **length**: actual number of elements.
    - If _vec.len()_ > _vec.capacity()_  -> capacity will automatically ↗, but elements will be reallocated -> slow -> _use Vec::with_capacity_ whenever possible to specify how big the vector is expected to get.
    - **Can only store values of the same type**. **Work around**: define an enum with different value types and store them it into a vector (pg. 134)
