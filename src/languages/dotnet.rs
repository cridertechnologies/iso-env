use std::fs;
use super::manager::LanguageManager;

pub struct DotnetManager;

impl LanguageManager for DotnetManager {
    fn setup_env(&self, name: &str, version: &str) {
        let global_json = format!("{{ \"sdk\": {{ \"version\": \"{}\" }} }}", version);
        let config_path = format!("{}/global.json", name);
        fs::write(config_path, global_json).expect("Failed to write global.json");
        println!(".NET {} environment set up for {}", version, name);
    }

    fn switch_env(&self, version: &str) {
        println!(".NET SDK switched to version {}", version);
    }
}