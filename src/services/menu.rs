use std::io::{self, Write};

use crate::services::console;
enum ConnectionType {
    None,
    SQLServer,
    Redis,
}


fn initialize_connection() -> ConnectionType {
    // Here you would typically prompt the user to select a connection type (SQL Server or Redis)
    // and establish the connection accordingly. For this example, we'll just return None.
    ConnectionType::None
}

fn initialize_program() {
    console::display_application_information();
    console::success("Testing Success Label", true);
    console::warning("Testing Warning Label", true);
    console::error("Testing Error Label", true);
    console::info("Loading Projects: {}", true); // TODO: Add Project Count

}


pub fn start_application() {
    console::clear_screen(); // Added Clear_screen to console scope
    initialize_program(); // Load needed actions to move through next step

    let connection_type: ConnectionType = ConnectionType::None; // Adds information towards console prefix; "> ", "SQL > ", "Redis > "

    println!("Welcome to the System Management Application!");

    let mut user_input = String::new();

    loop {
        let console_prefix = "";
        match &connection_type {
            ConnectionType::None => {
                console::console_prefix(&console_prefix.to_string()); // Reference the console_prefix variable here
            }
            ConnectionType::SQLServer => {
                console::console_prefix(&console_prefix.to_string()); // Reference the console_prefix variable here
            }
            ConnectionType::Redis => {
                println!("Connected to Redis.");

            }
        }

        io::stdout().flush().unwrap();
        user_input.clear();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        if user_input == "exit" {
            break;
        }

        let parts: Vec<&str> = user_input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = parts[1..].iter().map(|s| s.to_string()).collect();

        execute_command(command, args);
    }
}


 pub fn execute_command(command: &str, args: Vec<String>) {
    let command_list = vec![
        ("help", "Display this help message"),
        ("exit", "Exit the application"),
        ("connect_sql", "Connect to SQL Server"),
        ("connect_redis", "Connect to Redis"),
        ("disconnect_sql", "Disconnect from SQL Server"),
        ("disconnect_redis", "Disconnect from Redis"),
        ("execute_sql", "Execute SQL query"),
        ("execute_redis", "Execute Redis command"),
        ("clear", "Clear the console screen"),
        ("info", "Display application information"),
    ];
    // Implementation for executing commands
    // TODO: Implement the logic for executing commands based on the command string and arguments
    for (cmd, description) in &command_list {
        if cmd == &command {
            if args.is_empty() {
                console::info(&format!("Executing command: {}", cmd), true);
                console::info(&format!("Description: {}", description), true);
            } else {
                console::info(&format!("Executing command: {} with arguments: {:?}", cmd, args), true);
                console::info(&format!("Description: {}", description), true);
            }
            break;
        }
    }

    

}
