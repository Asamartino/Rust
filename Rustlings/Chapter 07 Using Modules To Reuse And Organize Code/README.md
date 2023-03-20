# Summary of chapter 7
**Module**: namespace that contains definitions of functions or types which can be visible outside their module (public) or not (private by default): mod: declares a new module 

**rule of modules filesystems**:
-	If a module named foo:
    - has no submodules -> put the declaration for foo in a file named foo.rs.
    - have submodules    -> put the declaration for foo in a file named foo/mod.rs.

**rule of visibility**: 
-	**If public**, can be accessed through any of its parent modules.
-	**If private**, can be accessed only by its immediate parent module and any of the parent’s child modules.

**use**: to avoid using the full path every time. Can also be used with enum:
```rust
use a::series::of;

fn main(){
    of::nested_modules;
}
///////////////////////////////////////////////////////////////////////
enum TafficLight {
    Red,
    Yellow,
    Green,
}
use TrafficLight::{Red,Yellow}; // to bring everything at once use TrafficLight::*
fn main(){
   let red = Red;
   let green = TrafficLight::green;
}
```

**\* = glob operator**: bring all visible items into scope at once. Convenient but might also pull more items than you expect -> naming conflicts.

**Paths are always relative to the current module** except with _use_ where it’s relative to the crate root by default.

Suppose you are in test and want to access _connect()_ in client0 -> 1 up in the module hierarchy:
- _::client::connect()_ start at the root of the module
- _super::client:.connect()_ go only 1 up and can be combined with _use_

![image](https://user-images.githubusercontent.com/61462365/226268113-1383f420-15ed-4207-8bad-431296ae3cee.png)

