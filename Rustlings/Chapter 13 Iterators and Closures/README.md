# Summary of chapter 13
Closures: anonymous functions that you can save in a variable or pass as arguments to other functions. Unlike functions, can capture their environment and access variables from the scope in which they’re defined. 
let expensive_closure = |num| {
    println!("claculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}
// expensive_closure contains the definition of an anonymous function // not its resulting value
// to call: expensive_closure(10)

	Don’t require you to annotate the types of the parameters or the return value like functions do (can add type annotations ↗ explicitness and clarity).  type will be inferred  can’t use different types (f.i. can’t call using an u32 as input and another time a String)  generic
fn add_one_v1    (x:u32) -> u32 {x+1}; // function
let add_one_v2 = |x:u32| -> u32 {x+1}; // closure
let add_one_v3 = |x|            {x+1}; // closure
let add_one_v4 = |x|             x+1 ; // closure
	Capture values from their environment in three ways: FnOnce, FnMut, Fn
	move: to take ownership of the values it uses
let equal_to_x = move |z| z==x;



- _String_ (wrapper over a _Vec\<u8\>_)
    - ``` “abc”.to_string() = String::from(“abc”)```
    - Adding:
      ```rust
      let s1 = String::from("Hello, ");
      let s2 = String::from("world!");
      let s3 = s1 + &s2;  // note s1 has been moved (can no longer be used) and s2 is coerced
      
      let s = format!("{}-{}-{}", s1,s2,s3); //returns a String and doesn’t take ownership of any of its parametes
      ```
    - **Doesn’t support indexing** as some characters > 1 byte. Can look at it as: **bytes** _s.bytes()_, **scalars** _s.chars()_ or **grapheme cluster** (≅ letters)

- _Hash map_: to look up data by using a key. 
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



