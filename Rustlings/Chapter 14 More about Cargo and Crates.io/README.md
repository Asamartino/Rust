# Summary of chapter 14
Cargo have many features: see [cargo doc](https://doc.rust-lang.org/cargo/). Two main profiles:
-	_cargo build_: **dev profile**, with good defaults for development.
-	_cargo build --release_: **release profile**, good defaults for release builds.<br/> 
Can modify the default profile by adding either _[profile.dev]_ or _[profile.release]_ in _Cargo.toml_.

documentation comment:
- ///  will generate HTML documentation by running _cargo doc_
- //!  typically use inside the crate root file or inside a module to document it as a whole

Can rearrange the structure of your public API using _pub use_

[crates.io](https://crates.io/): allow you to use package or share code with others .
- **Publishing a crate is permanent** -> code can't be overwritten or deleted. Version can be updated. 
- **Yanking**: (does not delete any code) prevents any future projects from adding a crate version while allowing all existing projects that depend on it to continue working. 
- **Workspace**: set of packages that share the same _Cargo.lock_ and output directory -> features that allow you to split up your package into multiple library crates. See book for example.
    - Only 1 _Cargo.lock_:  ensures all crates are using the same version of all dependencies (save space) + they will always be compatible with each other



