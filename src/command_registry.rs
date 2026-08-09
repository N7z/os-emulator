pub struct Command {
    pub name: String,
}

pub fn resolve(input: &str) -> Option<Command> {
    match input {
        "help" => Some(Command {
            name: String::from("help"),
        }),
        _ => None,
    }
}
