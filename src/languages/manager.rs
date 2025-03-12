pub trait LanguageManager {
    fn setup_env(&self, name: &str, version: &str);
    fn switch_env(&self, version: &str);
}