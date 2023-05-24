pub fn parse_generic_command(command: &str) -> (String, Vec<String>) {
    let mut flags = command.split_whitespace();
    let flags = flags.next().unwrap_or("");
    let args: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
    (flags.to_string(), args[1..].to_vec())
}
