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
    console::Success("Testing Success Label", true);
    console::Warning("Testing Warning Label", true);
    console::Error("Testing Error Label", true);
}


pub fn start_application() {
    clear_screen();
    initialize_program();

    let mut connection_type: ConnectionType = ConnectionType::None;

    println!("Welcome to the System Management Application!");

    let mut user_input = String::new();

    loop {
        print!("> ");        


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
        // let args = parts[1..].iter().map(|s| s.to_string()).collect();

        // execute_command(command, args);
    }
}


pub fn commands_list() {
    let mut map = std::collections::HashMap::new();
    map.insert("create_project", "Create a new project");
    map.insert("get_project_by_id", "Get a project by its ID");
    map.insert("update_project", "Update an existing project");
    map.insert("delete_project", "Delete a project by its ID");
    map.insert("list_projects", "List all projects");
    map.insert("activate_project", "Activate a project by its ID");
    map.insert("deactivate_project", "Deactivate a project by its ID");
    map.insert("restore_project", "Restore a deleted project by its ID");

    // Allow group and sub commands
    map.insert("connect", "Connect to a database (SQL Server or Redis)");
    map.insert("disconnect", "Disconnect from a database (SQL Server or Redis)");

    map.insert("execute", "Execute a command on a database (SQL Server or Redis)");
    map.insert("help", "Display this help message");
}

// pub fn execute_command(command: &str, args: Vec<String>) {
//     // Implementation for executing commands

    
// }

pub fn clear_screen() {
    // Clear the console screen
    print!("\x1B[2J\x1B[1;1H");
}
