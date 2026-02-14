use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

const LOG_FILE: &str = ".file-organizer-log.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub from: String,
    pub to: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationLog {
    pub operations: Vec<Operation>,
}

impl OperationLog {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn load(dir: &str) -> Self {
        let log_path = Path::new(dir).join(LOG_FILE);
        if log_path.exists() {
            let content = fs::read_to_string(&log_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        }
    }

    pub fn save(&self, dir: &str) {
        let log_path = Path::new(dir).join(LOG_FILE);
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize log");
        let mut file = fs::File::create(log_path).expect("Failed to create log file");
        file.write_all(json.as_bytes())
            .expect("Failed to write log file");
    }

    pub fn add_operation(&mut self, from: String, to: String) {
        self.operations.push(Operation {
            from,
            to,
            timestamp: Local::now().to_rfc3339(),
        });
    }
}

pub fn log_file_name() -> &'static str {
    LOG_FILE
}
