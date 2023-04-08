# Summary of Appendix D: Macros
Macros evolve quickly  the code you wrote might become outdated (although will still work thanks to Rust’s stability guarantees might be easier/better way to write macros). Useful resources:
	the reference
	the little book of rust macros

Macros: way of writing code that writes other code (known as metaprogramming) ≈ to functions but with additional capabilities:
	Can take a variable # of parameters.
println!("hello");
println!("hello {}", name);
let v: Vec<u32> = vec![1,2,3]; // vec! macro 
	Can implement a trait on a given type
	Must bring a macros into scope before you call them (whereas with functions can call them from anywhere).

