pub fn parse_generic_command(command: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    let command = parts.get(0).map_or("", |&cmd| cmd).to_string();
    let args = parts.into_iter().skip(1).map(String::from).collect();
    (command, args)
}
