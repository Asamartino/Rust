# Summary of chapter 7
Module: namespace that contains definitions of functions or types which can be visible outside their module (public) or not (private by default): mod: declares a new module 

rule of modules filesystems:
-	If a module named foo:
o	has no submodules  put the declaration for foo in a file named foo.rs.
o	have submodules     put the declaration for foo in a file named foo/mod.rs.

rule of visibility: 
-	If public, can be accessed through any of its parent modules.
-	If private, can be accessed only by its immediate parent module and any of the parent’s child modules.

use : to avoid using the full path every time. Can also be used with enum:
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
use TrafficLight::{Red,Yellow}; // to bring everything at once use                
                                   TrafficLight::*
fn main(){
   let red = Red;
   let green = TrafficLight::green;
}

* = glob operator: bring all visible items into scope at once, convenient but might also pull more items than you expect  naming conflicts.

Paths are always relative to the current module except with use where it’s relative to the crate root by default.

Suppose you are in test and want to access the connect() in client  1 up in the module hierarchy:
o	::client::connect() start at the root of the module
o	super::client:.connect() go only 1 up  can be combined with use
