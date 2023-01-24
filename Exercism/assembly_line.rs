// Instructions
// In this exercise you'll be writing code to analyze the production of an assembly line in a car factory. 
// The assembly line's speed can range from 0 (off) to 10 (maximum).

// At its lowest speed (1), 221 cars are produced each hour. The production increases linearly with the speed. 
// So with the speed set to 4, it should produce 4 * 221 = 884 cars per hour. However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded. 
// The following table shows how speed influences the success rate:

// 1 to 4: 100% success rate.
// 5 to 8: 90% success rate.
// 9 and 10: 77% success rate.
// You have two tasks.

// Calculate the production rate per hour

// Implement a method to calculate the assembly line's production rate per hour, taking into account its success rate:

// assembly_line::production_rate_per_hour(6)
// // Returns: 1193.4
// Note that the value returned is an f64.

// Calculate the number of working items produced per minute

// Implement a method to calculate how many working cars are produced per minute:

// assembly_line::working_items_per_minute(6)
// // Returns: 19
// Note that the value returned is an u32.

#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut sucess_rate = 0.0;
    match speed {
        1..=4 => sucess_rate = 1.0,
        5..=8 => sucess_rate = 0.9,
        9..=10 => sucess_rate = 0.77,
        _ => println!("Speed should range from 0 to 10"),
    }
    221.0 * speed as f64 * sucess_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
