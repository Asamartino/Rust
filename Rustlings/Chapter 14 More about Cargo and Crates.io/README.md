# Summary of chapter 14
Cargo have many features: see [cargo doc](https://doc.rust-lang.org/cargo/). Two main profiles:
-	cargo build: dev profile, with good defaults for development.
-	cargo build –release: release profile, good defaults for release builds.
Can modify the default profile by adding either [profile.dev] or [profile.release]   in Cargo.toml

crates.io: allow you to use package or share code with others .

documentation comment:
///  will generate HTML documentation by running cargo doc
//! : typically use inside the crate root file or inside a module to document the crate or the module as a whole

Can rearrange the structure of your public API using pub use
 
Publishing a crate is permanent  code can be overwritten or deleted. Can update the version 

Yanking: (does not delete any code) prevents any future projects from adding a crate version while allowing all existing projects that depend on it to continue working. 

Workspace: set of packages that share the same Cargo.lock and output directory  features that allow you to split up your package into multiple library crates. See book for common way example.
-	Only one Cargo.lock:  ensures all crates are using the same version of all dependencies (save space)  they will always be compatible with each other



