# Summary of Appendix D: Macros
**Macros** evolve quickly -> the code you wrote might become outdated (although will still work thanks to Rust’s stability guarantees might be easier/better way to write macros). Useful resources:
- [the reference](https://doc.rust-lang.org/reference/macros.html)
- [the little book of rust macros](https://danielkeep.github.io/tlborm/book/index.html)

**Macros**: way of writing code that writes other code (known as metaprogramming) ≈ to functions but with additional capabilities:
- Can take a variable # of parameters.
  ```rust
  println!("hello");
  println!("hello {}", name);
  let v: Vec<u32> = vec![1,2,3]; // vec! macro 
  ```
- Can implement a trait on a given type
- Must bring a macros into scope before you call them (whereas with functions can call them from anywhere).

Type of Macros:
- **Declarative macros**: ≈ _match_ (i.e. compare the input source code to patterns and replace the input source code with the code of the associated pattern)
  ```rust
  // look at pg. 503 for a detailed explanation 
  #[macro_export]
  macro_rules! vec {
      ( $( $x:expr ),* ) => { // only 1 pattern in this macro
          {
              let mut temp_vec = Vec::new();
              $(
                  temp_vec.push($x);
              )*
              temp_vec
          }
      }; // separate pattern with ;
  }
  ```
- **Procedural macros**: operate on the inputted code and produce an output (more similar to function). By convention, for a crate named _foo_, a custom derive procedural macro crate is called _foo_derive_
  ```rust
  extern crate proc_macro;
  extern crate syn;
  #[macro_use]
  extern crate quote;

  use proc_macro::TokenStream;

  #[proc_macro_derive(HelloMacro)]
  pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
      // Construct a string representation of the type definition
      let s = input.to_string()

      // Parse the string represnetation
      let ast = syn::parse_derive_input(&s).unwrap();

      //Build the impl
      let gen = impl_hello_macro(&ast);  // will vary depending on the goal of your procedural macro

      // Return the generated impl
      gen.parse().unwrap()
  }

  fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
      let name = &ast.ident;
      quote! {
          impl HelloMacro for #name {
              fn hello_macro() {
                  println!("Hello, Macro! My name is {}!", stringify!(#name));
              }
          }
      }
  }
  ```

