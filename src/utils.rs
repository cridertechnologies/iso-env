use std::process::{Command, exit};

/// Detects the OS and returns the appropriate package manager command.
fn get_install_command() -> (&'static str, Vec<&'static str>){
    if cfg!(target_os = "windows"){
        return ("winget", vec!["install"]);
    } else if cfg!(target_os = "macos") {
        return ("brew", vec!["install"]);
    } else if cfg!(target_os = "linux") {
        return ("apt", vec!["install", "-y"]); // Assumes Debian-based.
    } else {
        panic!("Unsupported OS");
    }
}

/// Checks if a tool is installed by running its version command.
pub fn is_tool_installed(check_command: &str, check_arg: &str) -> bool {
    Command::new(check_command)
        .arg(check_arg)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Installs a missing tool.
pub fn ensure_tool_installed(tool_name: &str, package_name: &str) {
    if is_tool_installed(tool_name, "--version") {
        println!("✅ {} is already installed.", tool_name);
        return;
    }

    let (install_command, install_args) = get_install_command();
    let mut args = install_args.clone();
    args.push(package_name);

    println!("⚠️  {} not found. Installing using {}...", tool_name, install_command);

    let output = Command::new(install_command)
        .args(&args)
        .output()
        .expect(&format!("❌ Failed to install {}", tool_name));

    if output.status.success() {
        println!("✅ {} installed successfully.", tool_name);
    } else {
        println!("❌ Failed to install {}. Please install it manually.", tool_name);
        exit(1);
    }
}
