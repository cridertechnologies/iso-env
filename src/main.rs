mod languages;
mod utils;

use clap::{Parser, Subcommand};
use std::process::exit;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use languages::get_language_managers;
use languages::manager::LanguageManager;

#[derive(Parser)]
#[command(name = "iso-env")]
#[command(version = "1.0")]
#[command(about = "Project Isolation Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new isolated environment
    Create { name: String, lang: String, version: Option<String> },

    /// Switch to an existing environment
    Use { name: String },

    /// List all managed environments
    List
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    project_name: String,
    language: String,
    version: String,
    isolation: String
}

fn main() {
    let cli = Cli::parse();
    let managers = get_language_managers();

    match cli.command {
        Commands::Create { name, lang, version } => {
            let version = version.unwrap_or_else(|| "latest".to_string());
            create_project(name, lang, version, &managers);
        },
        Commands::Use { name } => switch_environment(name),
        Commands::List => list_projects(),
    }
}

fn create_project(name: String, lang: String, version: String, managers: &std::collections::HashMap<String, Box<dyn LanguageManager>>) {
    let path = format!("./{}", name);
    if Path::new(&path).exists() {
        println!("Project {} already exists!", name);
        return;
    }
    
    fs::create_dir_all(&path).expect("Failed to create project directory");

    let config = ProjectConfig {
        project_name: name.clone(),
        language: lang.clone(),
        version,
        isolation: lang.clone(),
    };

    let config_path = format!("{}/.envconfig.json", path);
    let json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(config_path, json).expect("Failed to write config");

    println!("Project {} created with {} version {}!", name, &lang, config.version);

    if let Some(manager) = managers.get(&lang) {
        manager.setup_env(&name, &config.version);
    } else {
        println!("Unsupported language: {}", lang);
    }
}

fn switch_environment(name: String) {
    let config_path = format!("./{}/.envconfig.json", name);
    if !Path::new(&config_path).exists() {
        println!("No environment found for {}", name);
        exit(1);
    }

    let config: ProjectConfig = serde_json::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();

    if let Some(manager) = get_language_managers().get(&config.language) {
        manager.switch_env(&config.version);
        println!("Switched to environment {}", name);
    } else {
        println!("Unsupported language: {}", config.language);
    }
}

fn list_projects() {
    let entries = fs::read_dir("./").expect("Failed to read directory");
    println!("Managed Projects:");
    for entry in entries {
        let entry = entry.unwrap();
        if entry.path().join(".envconfig.json").exists() {
            println!("- {}", entry.file_name().to_string_lossy());
        }
    }
}