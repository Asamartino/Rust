# Summary of chapter 8
Collections (contains multiple values)  = **pointers with data stored on the heap** (≠ array or tuple) -> amount of data does not need to be known at compile time -> can grow or shrink as program run. Can be extended using _extend()_:
- _Vec\<T\>_
    - ``` vec!(..) = vec![..] ```
    - To access an element:
      ```rust
      let v = vec[1,2,3,4,5];
      let third: &i32 = &v[100];             // programs crash
      let third: Option<&i32> = v.get(100);  // returns None
      ```
    - To iterate:
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

- String (wrapper over a _Vec\<u8\>_)
    - ``` “abc”.to_string() = String::from(“abc”)```
    - Adding:
      ```rust
      let s1 = String::from("Hello, ");
      let s2 = String::from("world!");
      let s3 = s1 + &s2;  // note s1 has been moved (can no longer be used) and s2 is coerced
      
      let s = format!("{}-{}-{}", s1,s2,s3); //returns a String and doesn’t take ownership of any of its parametes
      ```
    - **Doesn’t support indexing** as some characters > 1 byte. Can look at it as: **bytes** _s.bytes()_, **scalars** _s.chars()_ or **grapheme cluster** (≅ letters)

- Hash map: to look up data by using a key. 
    - Need to be imported it via: ```use std::collections::HashMap;```
    - Keys can be of any type, but all must have the same type (same for values). 
    - **For values that:**
        - **implement the Copy trait: values are copied**
        - **for owned values: values will be moved and the hashmap will be the owner**.
    - Different ways to iterate over them (see library):
    - Any type that implements _Eq_ and _Hash_ traits can be a key in HashMap
    - Different way of updating a hashmap -> pg 145
    - HashMap uses a specific hashing algorithm, very competitive for medium sized keys, but for small keys or large keys might be outperformed by others hashing algorithms.
      ```rust
      let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
      ];//it’s an array
      let mut teams_map1 = HashMap::new();
      for team in &teams {
          teams_map1.insert(team.0, team.1);
      }
      let teams_map2: HashMap<_,_> = teams.into_iter().collect();
      let teams_map3 = HashMap::from(teams);
      ```
    - HashMaps are growable but can also shrink themselves when they have excess space.



