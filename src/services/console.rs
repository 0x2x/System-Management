
// Source - https://stackoverflow.com/a/73494635
// Posted by Rice7th
// Retrieved 2026-07-18, License - CC BY-SA 4.0

pub fn color_test() {
  println!("Red: \x1b[41m  \x1b[0m");
  println!("Yellow: \x1b[43m  \x1b[0m");
  println!("Green: \x1b[42m  \x1b[0m");

  // Combine them to create complex results
  println!("\x1b[32m\x1b[44mGreen Text with blue background\x1b[0m");
}

// Console Utilities

pub fn display_application_information() {
    let version = "0.1.0";
    let status = "Ready";
    let configuration_loaded = true;
    let services_initialized = true;
    let plugins_registered = true;
    let logging_enabled = true;


    println!("╭──────────────────────────────────────────────────────────────╮");
    println!("│                                                              │");
    println!("│   🦀  System Management                                      │");
    println!("│                                                              │");
    println!("│   High-performance system administration framework           │");
    println!("│                                                              │");
    println!("│   Version : {}                                               │", version);
    println!("│   Language: Rust                                             │");
    if status == "Ready" {
        println!("│   Status  : \x1b[32m{}\x1b[0m                                               │", status); // Green
    } else if status == "Error" {
        println!("│   Status  : \x1b[31m{}\x1b[0m                                               │", status); // Red
    } else if status == "Warning" {
        println!("│   Status  : \x1b[33m{}\x1b[0m                                               │", status); // Yellow
    } else {
        println!("│   Status  : {}                                               │", status);
    }
    println!("│                                                              │");
    println!("├──────────────────────────────────────────────────────────────┤");
    println!("│                                                              │");

    if configuration_loaded {
        println!("│  ✓ Configuration Loaded                                      │");
    }else {
        println!("│  ✗ Configuration Not Loaded                                  │");
    }
    if services_initialized {
        println!("│  ✓ Services Initialized                                      │");
    }
    else {
        println!("│  ✗ Services Not Initialized                                  │");
    }
    if plugins_registered {
        println!("│  ✓ Plugins Registered                                        │");
    }else {
        println!("│  ✗ Plugins Not Registered                                    │");
    }
    if logging_enabled {
        println!("│  ✓ Logging Enabled                                           │");
    }else {
        println!("│  ✗ Logging Not Enabled                                       │");
    }
    println!("│                                                              │");
    println!("╰──────────────────────────────────────────────────────────────╯");
}

pub fn clear_screen() {
    // Clear the console screen
    print!("\x1B[2J\x1B[1;1H");
}

pub fn pause() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

#[cfg(target_os = "windows")]
fn windows_console_prefix(connection: &String) {
    let user_name = std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());
    let computer_name = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    if connection.is_empty() {
        print!("\x1b[34m[{}@{}]\x1b[0m > ", user_name, computer_name); // Blue
    } else {
        print!("\x1b[32m[{}]\x1b[0m > ", connection); // Green
    }
}

#[cfg(target_os = "linux")]
fn unix_console_prefix(connection: &String) {
    let user_name = std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string());

    if connection.is_empty() {
        print!("\x1b[34m[{}@{}]\x1b[0m > ", user_name, computer_name); // Blue
    } else {
        print!("\x1b[32m[{}]\x1b[0m > ", connection); // Green
    }
}

#[cfg(target_os = "macos")]
fn macos_console_prefix(connection: &String) {
    let user_name = std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    let computer = std::env::var("HOSTNAME").unwrap_or_else(|_| "Unknown".to_string());
    if connection.is_empty() {
        print!("\x1b[34m[{}@{}]\x1b[0m > ", user_name, computer); // Blue
    } else {
        print!("\x1b[32m[{}]\x1b[0m > ", connection); // Green
    }
}


#[warn(unused)]
pub fn console_prefix(connection: &String) {
    // TODO: Implement a more robust console prefix system that adapts to different operating systems and connection types.
    let operating_system = std::env::consts::OS;
    print!("\x1b[34m[{}{}]\x1b[0m > ", connection, operating_system); // Blue

    // print!("\x1b[34m[{}]\x1b[0m > ", operating_system); // Blue
    // if cfg!(target_os = "windows") {
    //     windows_console_prefix(connection);
    // } else if cfg!(target_os = "linux") {
    //     unix_console_prefix(connection);
    // } else if cfg!(target_os = "macos") {
    //     macos_console_prefix(connection);
    // } else {
    //     println!("Unsupported operating system: {}", operating_system);
    // }

}

// Console Colors Foreground
pub fn success(msg: &str, labled: bool) {
    if labled {
        println!("\x1b[32m[Success]\x1b[0m {}", msg); // Green
    } else {
        println!("\x1b[32m{}\x1b[0m", msg); // Green
    }
}

pub fn warning(msg: &str, labled: bool) {
    if labled {
        println!("\x1b[33m[Warning]\x1b[0m {}", msg); // Yellow
    } else {
        println!("\x1b[33m{}\x1b[0m", msg); // Yellow
    }
}

pub fn error(msg: &str, labled: bool) {
    if labled {
        println!("\x1b[31m[Error]\x1b[0m {}", msg); // Red
    } else {
        println!("\x1b[31m{}\x1b[0m", msg); // Red
    }
}


pub fn info(msg: &str, labled: bool) {
    if labled {
        println!("\x1b[34m[Info]\x1b[0m {}", msg); // Blue
    } else {
        println!("\x1b[34m{}\x1b[0m", msg); // Blue
    }
}

pub fn debug(msg: &str, labled: bool) {
    if labled {
        println!("\x1b[35m[Debug @ {}]\x1b[0m {}", file!(), msg); // Magenta
    } else {
        println!("\x1b[35m{}\x1b[0m", msg); // Magenta
    }
}
