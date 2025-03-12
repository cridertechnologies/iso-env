use std::process::Command;
use super::manager::LanguageManager;

pub struct PythonManager;

impl LanguageManager for PythonManager {
    fn setup_env(&self, name: &str, version: &str) {
        let output = Command::new("pyenv")
        .arg("install")
        .arg(version)
        .output()
        .expect("Failed to install Python version");

        println!("{}", String::from_utf8_lossy(&output.stdout));

        let venv_path = format!("{}/.venv", name);
        Command::new("python")
            .arg("-m")
            .arg("venv")
            .arg(venv_path)
            .output()
            .expect("Failed to create Python virtual environment");

        println!("Python {} virtual environment created for {}", version, name);
    }

    fn switch_env(&self, version: &str) {
        let output = Command::new("pyenv")
        .arg("local")
        .arg(version)
        .output()
        .expect("Failed to switch Python version");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
