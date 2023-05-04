# Summary of chapter 17
An object packages both data and the procedures (called methods or operations) that operate on that data.
- **Encapsulation**: implementation details of an object aren’t accessible to code using that object. Use _pub_ to decide what should be public (private by default):
The struct is public but if fields are private -> can only modify the fields using public functions
  ```rust
  pub struct AveragedCollection {
      list: Vec<i32>,
      average: f64,
  }
  ```

**Polymorphism**: if objects share certain characteristics, they can be substituted for each other at runtime.

**Inheritance** (not in Rust): mechanism whereby an object can inherit from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.\
Rust uses **trait objects**: created by specifying a pointer (e.g. `&`, `Box<T>`) and specifying the relevant trait. Allow for multiple concrete types to fill in for the trait object at runtime (≠ generics which substitute one concrete type at a time).
  ```rust
  pub trait Draw {
      fn draw(&self);
  }
  pub struct Screen {
      pub components: Vec<Box<dyn Draw>>, // = trait object: any type inside a Box that implements the Draw trait     
  }
  ```
- By using trait object, Rust must use dynamic dispatch  runtime cost
- **Trait must be object safe** because once used, Rust no longer knows the concrete type that’s implementing the trait.
  - A trait is **object safe** if all the methods defined in the trait have the following properties:
    - The return type isn’t Self
    - There are no generic type parameters

**State pattern**: crux of the pattern is that a value has some internal state (represented by a set of state objects), and the value’s behavior Δ based on the internal state.

Rust is capable of implementing object-oriented design patterns and other patterns -> trade-offs (see examples pg 376, 384)




