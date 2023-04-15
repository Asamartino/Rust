# Summary of Appendix D: Macros
**Macros** evolve quickly -> the code you wrote might become outdated (although it will still work, thanks to Rust's stability guarantees). There might be an easier or better way to write macros. Useful resources:
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

## Summary of [MacroKata](https://github.com/tfpk/macrokata)

In addition of the [very good exercises](https://github.com/tfpk/macrokata/tree/main/exercises), here is a short summary of the most important concepts:

1. 	**Rust's macros allow you to break many of the syntax rules**. For example, Rust does not allow functions with Δ# of arguments (also known as *variadic function*) -> can’t use _println_ -> use it as a macro. Before _println!_ is compiled, Rust rewrites it into a function which takes a single array of arguments.
    ```rust
    // to run a macro
    macro_name!(...) or macro_name!{...} or macro_name![...]
    ```
2. Macros are very similar to a _match_ statement because they find the first match and take action based on that; but you're matching on **tokens**.
![image](https://user-images.githubusercontent.com/61462365/232204734-3de7794b-9144-4533-97e1-9b6accdbe08c.png)
3. **Metavariables**: capture a particular part of the text inside the macro's brackets, and let you reuse it.
    ```rust
    macro_rules! do_thing {
        (print $metavar:literal) => {
            println!("{}", $metavar)
        };
    }

    fn main() {
        do_thing!(print 3);
    }
    ```
4. 	In general, every metavariable is of the form `$<name>:<fragspec>`:
    - `<name>`: name of the metavariable, 
	  
    - `<fragspec>`: "Fragment Specifier", what sort of fragment you intend to match. 
	      - `literal`: 'a', 3, "hello"
	      - `expr`: for expression
	      - `stmt`: ≈ `expr` but allow Rust statements too (f.i. `let`)
	 
  	**Follow-set Ambiguity Rules**: which tokens are allowed to follow a metavariable:
	   - For `literal`: anything can follow it.
	   - For `expr` and `stmt`: can only be followed by  `=>`  or `,` or `;`

5. Other fragment specifiers (look [here](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables) for more):
    - `ident`: identifier, like a variable name, can be followed by anything.
    - `block`: block expression (curly braces, and their contents), can be followed by anything.
    - `ty`: a type, can only be followed by: `=>` `,` `=` `|` `;` `:` `>` `>>` `[` `{` `as` `where` or a block metavariable.
6. Macro repetitions: to match and manipulate repeated patterns. Consists of:
    - A group of tokens that we want to match repeatedly.
    - Optionally, a separator token (which tells the parser what to look for between each match).
    - Either `+`, `*` or `?`, which says how many times to expect a match. 
      - `+`: at least once. 
      - `*`: any number of times, including 0 times
      - `?`: either 0 or 1 time
    ```rust
    #[macro_export]
    macro_rules! and_text {
    $(the $my_literal:literal)and+ => { // and is a separator could also use , it’s optional
            {
                let mut my_vec = Vec::new();
                $(my_vec.push($my_literal);)*
                my_vec
            }
        }
    }
    ...
    and_text!(the "lion" and the "witch" and the "wardrobe");
    ```

7. Can use more than one metavariable in macro repetitions. "_However, in the transcriber, if multiple metavariables appear in the same repetition they must be bound to the same number of fragments_” see [the rust reference](https://doc.rust-lang.org/reference/macros-by-example.html#transcribing).
8. Nested repetition: be sure to clearly refer the desried metavariable.
9. [Transcribing](https://doc.rust-lang.org/reference/macros-by-example.html#transcribing): “_When a macro is invoked, the macro expander looks up macro invocations by name, and tries each macro rule in turn. It transcribes the first successful match; if this results in an error, then future matches are not tried. When matching, no lookahead is performed; if the compiler cannot unambiguously determine how to parse the macro invocation one token at a time, then it is an error_”.


