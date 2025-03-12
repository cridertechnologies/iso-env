use std::process::Command;
use super::manager::LanguageManager;
use crate::utils::ensure_tool_installed;

pub struct RustManager;

impl LanguageManager for RustManager {
    fn setup_env(&self, name: &str, version: &str) {
        ensure_tool_installed("rustup", "rustup");

        let output = Command::new("rustup")
        .arg("install")
        .arg(version)
        .output()
        .expect("Failed to install Rust version");

        println!("{}", String::from_utf8_lossy(&output.stdout));

        Command::new("cargo")
            .arg("new")
            .arg(name)
            .output()
            .expect("Failed to create Rust project");

        println!("Rust project {} created with version {}", name, version);
    }

    fn switch_env(&self, version: &str) {
        let output = Command::new("rustup")
        .arg("override")
        .arg("set")
        .arg(version)
        .output()
        .expect("Failed to switch Rust version");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}