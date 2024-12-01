use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    pub network: String,
    pub api_key: String,
    pub default_change_address: Option<String>,
}

impl AppConfig {
    /// Load configuration from file or environment variables
    pub fn load(config_file: &str) -> Self {
        dotenv().ok(); // Load .env file if it exists

        if Path::new(config_file).exists() {
            let content = fs::read_to_string(config_file).expect("Failed to read config file");
            let mut config: Self = toml::from_str(&content).expect("Failed to parse config file");

            // Check if API key is present, else try to load from .env or prompt
            if config.api_key.is_empty() {
                println!("GetBlock API key is missing in the config file.");
                config.api_key = env::var("GETBLOCK_API_KEY").unwrap_or_else(|_| {
                    println!("API key not found in environment variables.");
                    config.prompt_for_api_key()
                });
                config.save(config_file);
            }
            config
        } else {
            println!("No config file found. Creating a new one.");
            let mut config = Self::default();

            // Attempt to load API key from environment variables or prompt
            config.api_key = env::var("GETBLOCK_API_KEY").unwrap_or_else(|_| {
                println!("API key not found in environment variables.");
                config.prompt_for_api_key()
            });
            config.save(config_file);
            config
        }
    }

    /// Save configuration to file
    pub fn save(&self, config_file: &str) {
        let content = toml::to_string(self).expect("Failed to serialize config");
        fs::write(config_file, content).expect("Failed to write config file");
    }

    /// Prompt the user to enter their GetBlock API key
    fn prompt_for_api_key(&mut self) -> String {
        print!("Please enter your GetBlock API key: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
