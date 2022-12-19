// Instructions
// Recite the lyrics to that beloved classic, that field-trip favorite: 99 Bottles of Beer on the Wall.

// Note that not all verses are identical.

// 99 bottles of beer on the wall, 99 bottles of beer.
// Take one down and pass it around, 98 bottles of beer on the wall.

// 98 bottles of beer on the wall, 98 bottles of beer.
// Take one down and pass it around, 97 bottles of beer on the wall.

// ...

// 2 bottles of beer on the wall, 2 bottles of beer.
// Take one down and pass it around, 1 bottle of beer on the wall.

// 1 bottle of beer on the wall, 1 bottle of beer.
// Take it down and pass it around, no more bottles of beer on the wall.

// No more bottles of beer on the wall, no more bottles of beer.
// Go to the store and buy some more, 99 bottles of beer on the wall.

// For bonus points
// Did you get the tests passing and the code clean? If you want to, these are some additional things you could try:

// Remove as much duplication as you possibly can.
// Optimize for readability, even if it means introducing duplication.
// If you've removed all the duplication, do you have a lot of conditionals? Try replacing the conditionals with polymorphism, if it applies in this language. How readable is it?
// Then please share your thoughts in a comment on the submission. Did this experiment make the code better? Worse? Did you learn anything from it?


pub fn verse(n: u32) -> String {
    let mut str = String::from("");
    if n == 0 {
        str.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    if n == 1 {
        str.push_str("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    if n == 2 {
        str.push_str("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    }
    if n > 2 {
        let str2 = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",n,n, n-1);
        str.push_str(&str2);
    }
    return str;
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::from("");
    println!("in sing");
    for n in ((end + 1)..(start + 1)).rev() {
        song.push_str(&verse(n));
        song.push_str("\n");
    }
    song.push_str(&verse(end));
    return song;
}

fn main() {
    assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}
