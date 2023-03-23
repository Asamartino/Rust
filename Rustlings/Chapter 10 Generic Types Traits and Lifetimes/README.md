# Summary of chapter 10
**Generics allow code to operate on abstract type**. Use generics to create definitions for items like function signatures or structs, which can be used with many different concrete data types. In the same way that you recognize duplicated code (which is tedious and error prone), you’ll start to recognize duplicated code that can use generics (f.i. function, struct, or enum that differ only in the type of the values they hold). By convention T (short for “type) but can use any parameter
```rust
fn largest<T> (list: &[T]) -> T {} // fn ...<T> indicates that it is a generic function
```
- use as many generic type parameters as you want -> but using more than a few makes your code hard to read -> could indicate that your code needs restructuring into smaller pieces.
- Generic methods on structs and enums: 
    - By declaring \<T\> as a generic type after _impl_, Rust identifies that the type in the angle brackets is a generic type.
      ```rust
      struct Point<T>{ x:T, y:T,}
      impl<T> Point<T>{
           fn x(&self) ->&T{ 
              &self.x
          }
      }
      impl Point<f32>{  //implement only for Point<f32>
          fn distance_from_origin(&self) -> f32{
              (self.x.powi(2) + self.y.powi(2)).sqrt()
          }
      }
      ```
    - Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signature:

      ```rust
      impl<T,U> Point<T,U>{ 
      fn mixup<V,W>(self, other: Point<V,W>) -> Point<V,W> { 
              Point{
                  x:self.x,
                  y:other.y,
              }
          }
      }
      
      ```
    - V,W are declared after the function because they're only relevant to the method
- **Monomophization**: process of turning generic code into specific code by filling in the concrete types that are used when compiled -> **makes Rust’s generic extremely efficient at runtime**.


**Trait**: functionality a particular type has and can share with other types. Used to:
- define shared behavior in an abstract way 
  ```rust
  pub trait Summary {
      fn summarize(&self) -> String;
  
      fn summarization(&self) -> String{ //default implementation
          String::from(("Read more..)"))
      }
  }
  
  impl Summary for NewsArticle{
      fn summarize(&self) -> String{
          format!("{},{},({})", self.headline,self.author, self.location)
      }
  }
  ```
    - Each type implementing the _summarize_ trait must provide its own custom behavior for it. 
    - Compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature.
- **Restriction**: can implement a trait on a type only if either the trait or the type is local to our crate -> can’t implement external trait on external types (f.i. Display trait on _Vec\<T\>_) -> **orphan rule**: ensure that other people’s code can’t break your code and vice versa. 
- **Trait bounds**: to constrain generic types to ensure the type will be limited to those that implement a particular trait and behavior.  Can specify multiple trait bounds on a generic type using “+” or with _where_.
  ```rust
  fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) –> i32 {}
  // could also be written as
  fn some_function<T, U>(t: T, u: U) -> i32
      where T: Display + Clone,
      U: Clone + Debug
  {}
   ```
- **Blanket implementation**: implementations of a trait on any type that satisfies the trait bounds

