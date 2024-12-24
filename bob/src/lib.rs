pub fn reply(message: &str) -> &str {
    match message.trim() {
        question if question.ends_with('?') => {
            if is_yelling(message) {
                return "Calm down, I know what I'm doing!";
            }
            return "Sure.";
        }
        y  if is_yelling(y) => "Whoa, chill out!",
        "" => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

fn is_yelling(message: &str) -> bool {
    message.chars().any(char::is_alphabetic) && message == message.to_uppercase()
}
