#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub arguments: Vec<String>,
}

pub fn new(name: &str, description: &str, arguments: Vec<String>) -> Self {
    Command {
        name: name.to_string(),
        description: description.to_string(),
        arguments,
    }
}

pub fn display_help(commands: &[Command]) {
    println!("Available commands:");
    for command in commands {
        println!("{}: {}", command.name, command.description);
    }
}

pub fn execute_command(command: &Command) {
    println!("Executing command: {}", command.name);
    // Here you would implement the logic to execute the command based on its name and arguments.
}
