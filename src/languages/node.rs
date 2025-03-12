use std::process::Command;
use super::manager::LanguageManager;

pub struct NodeManager;

impl LanguageManager for NodeManager {
    fn setup_env(&self, name: &str, version: &str) {
        let output = Command::new("nvm")
        .arg("install")
        .arg(version)
        .output()
        .expect("Failed to install Node.js version");

        println!("{}", String::from_utf8_lossy(&output.stdout));

        let project_path = format!("./{}", name);
        Command::new("npx")
            .arg("create-react-app")
            .arg(&project_path)
            .output()
            .expect("Failed to create React app");

        println!("Node.js/React project created in {}", project_path);
    }

    fn switch_env(&self, version: &str) {
        let output = Command::new("nvm")
        .arg("use")
        .arg(version)
        .output()
        .expect("Failed to switch Node.js version");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
