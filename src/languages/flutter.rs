use std::process::Command;
use super::manager::LanguageManager;

pub struct FlutterManager;

impl LanguageManager for FlutterManager {
    fn setup_env(&self, name: &str, version: &str) {
        let output = Command::new("fvm")
        .arg("install")
        .arg(version)
        .output()
        .expect("Failed to install Flutter version");

        println!("{}", String::from_utf8_lossy(&output.stdout));

        let project_path = format!("./{}", name);
        Command::new("flutter")
            .arg("create")
            .arg(&project_path)
            .output()
            .expect("Failed to create Flutter project");

        println!("Flutter project created in {}", project_path);
    }

    fn switch_env(&self, version: &str) {
        let output = Command::new("fvm")
        .arg("use")
        .arg(version)
        .output()
        .expect("Failed to switch Flutter version");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
