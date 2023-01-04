// Instructions
// Bob is a lackadaisical teenager. In conversation, his responses are very limited.

// Bob answers 'Sure.' if you ask him a question, such as "How are you?".

// He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).

// He answers 'Calm down, I know what I'm doing!' if you yell a question at him.

// He says 'Fine. Be that way!' if you address him without actually saying anything.

// He answers 'Whatever.' to anything else.

// Bob's conversational partner is a purist when it comes to written communication and always follows normal rules regarding sentence punctuation in English.

// V1
// pub fn reply(message: &str) -> &str {
//     let mut only_whitespace = true;
//     let mut only_capital = true;
//     let mut only_numeric = true;
//     let question = message.trim().ends_with('?');

//     for char in message.chars(){
//         if char.is_lowercase(){
//             only_capital = false;
//         }
//         if !char.is_whitespace(){
//             only_whitespace = false;
//         }
//         if char.is_alphabetic(){
//             only_numeric = false;
//         }
//     }

//     if message.is_empty() || only_whitespace{
//         return "Fine. Be that way!";
//     }
//     if question {
//         if only_capital && !only_numeric{
//             return "Calm down, I know what I'm doing!";
//         } else{
//             return "Sure.";
//         }
//     }
//     if only_capital && !only_numeric{
//         return "Whoa, chill out!"
//     }
//     return "Whatever.";
// }

// V2 with match
pub fn reply(message: &str) -> &str {
    let mut only_whitespace = true;
    let mut only_capital = true;
    let mut only_numeric = true;
    let question = message.trim().ends_with('?');

    for char in message.chars() {
        if char.is_lowercase() {
            only_capital = false;
        }
        if !char.is_whitespace() {
            only_whitespace = false;
        }
        if char.is_alphabetic() {
            only_numeric = false;
        }
    }
    match message {
        m if message.is_empty() || only_whitespace => "Fine. Be that way!",
        m if question && only_capital && !only_numeric => "Calm down, I know what I'm doing!",
        m if question => "Sure.",
        m if only_capital && !only_numeric => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
