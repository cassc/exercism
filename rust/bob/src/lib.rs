fn all_uppercase(s: &str) -> bool{
    let with_letter = s.chars().any(|c| c.is_alphabetic());
    with_letter && s.to_uppercase() == s
}


pub fn reply(message: &str) -> &str {
    match message.trim(){
        x if x.is_empty() => "Fine. Be that way!" ,
        x if x.ends_with("?") && all_uppercase(x) => "Calm down, I know what I'm doing!",
        x if x.ends_with("?") => "Sure.",
        x if all_uppercase(x) => "Whoa, chill out!",
        _ =>"Whatever.",
    }
}
