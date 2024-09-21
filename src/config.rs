use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use ini::Ini;
use dirs;

pub struct Config {
    pub author: String,
    pub courses: HashMap<String, String>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let mut config_path = PathBuf::from(home_dir);
        config_path.push(".cdocrc");

        // if not found, create one
        if !config_path.exists() {
            File::create(&config_path).map_err(|_| "Could not create config file")?;
        }

        let ini = Ini::load_from_file(&config_path).map_err(|_| "Could not load config file")?;

        // Extract author from settings, fallback to "John Doe"
        let author = ini.get_from(Some("Settings"), "Author")
            .unwrap_or("John Doe")
            .to_string();

        // Extract courses section
        let mut courses = HashMap::new();
        if let Some(section) = ini.section(Some("courses")) {
            for (key, value) in section.iter() {
                courses.insert(key.to_string(), value.to_string());
            }
        }

        Ok(Config { author, courses })
    }

    pub fn get_course_name(&self, course_code: &str) -> String {
        self.courses
            .get(course_code)
            .unwrap_or(&course_code.to_string())
            .to_string()
    }
}
