// Instructions
// Given a moment, determine the moment that would be after a gigasecond has passed.

// A gigasecond is 10^9 (1,000,000,000) seconds.

// If you're unsure what operations you can perform on PrimitiveDateTime take a look at the time crate which is listed as a dependency in the Cargo.toml file for this exercise.

use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}
