# Summary of chapter 16

The Rust team discovered that ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems. By using them may concurrency errors become compile-time errors  fearless concurrency.

**Threads**: provide a way to split a program into multiple independent tasks that can be executed concurrently, potentially speeding up the program execution but also adds complexity.
Rust standard library only provides an implementation of 1:1 threading.
