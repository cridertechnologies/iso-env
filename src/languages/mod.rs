pub mod manager;
pub mod python;
pub mod node;
pub mod flutter;
pub mod dotnet;

use std::collections::HashMap;
use flutter::FlutterManager;
use manager::LanguageManager;
use dotnet::DotnetManager;
use node::NodeManager;
use python::PythonManager;

/// Register all supported languages
pub fn get_language_managers() -> HashMap<String, Box<dyn LanguageManager>> {
    let mut managers = HashMap::new();

    managers.insert("dotnet".to_string(), Box::new(DotnetManager) as Box<dyn LanguageManager>);
    managers.insert("flutter".to_string(), Box::new(FlutterManager) as Box<dyn LanguageManager>);
    managers.insert("node".to_string(), Box::new(NodeManager) as Box<dyn LanguageManager>);
    managers.insert("react".to_string(), Box::new(NodeManager) as Box<dyn LanguageManager>);
    managers.insert("angular".to_string(), Box::new(NodeManager) as Box<dyn LanguageManager>);
    managers.insert("python".to_string(), Box::new(PythonManager) as Box<dyn LanguageManager>);
    managers
}