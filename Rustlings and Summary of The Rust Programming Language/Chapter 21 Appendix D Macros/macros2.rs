// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

macro_rules! my_macro { // must bring macros into scope before you call them in a file
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}


