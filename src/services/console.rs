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


// Console Colors Foreground
pub fn Success(msg: &str, labled: bool) {
    if (labled) {
        println!("\x1b[32m[Success]\x1b[0m {}", msg); // Green
    } else {
        println!("\x1b[32m{}\x1b[0m", msg); // Green
    }
}

pub fn Warning(msg: &str, labled: bool) {
    if (labled) {
        println!("\x1b[33m[Warning]\x1b[0m {}", msg); // Yellow
    } else {
        println!("\x1b[33m{}\x1b[0m", msg); // Yellow
    }
}

pub fn Error(msg: &str, labled: bool) {
    if (labled) {
        println!("\x1b[31m[Error]\x1b[0m {}", msg); // Red
    } else {
        println!("\x1b[31m{}\x1b[0m", msg); // Red
    }
}
