use chrono::Utc;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

/// Holds the settings for the logger.
pub struct Logger {
    log_to_stdout: bool,
    log_to_file: bool,
    log_file_path: Option<PathBuf>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            log_to_stdout: true,
            log_to_file: false,
            log_file_path: None,
        }
    }

    pub fn log_to_file(&mut self, file_path: &Path) {
        if !file_path.exists() {
            match File::create(file_path) {
                Ok(_) => self.info(format!("Create log file \"{}\"", file_path.display())),
                Err(_) => {
                    self.error(format!(
                        "Could not create log file \"{}\"",
                        file_path.display()
                    ));
                    self.warning("Skip logging to file".to_string());
                    return; // Exit and skip logging to file
                }
            }
        }
        self.log_to_file = true;
        self.log_file_path = Some(file_path.to_path_buf());
    }

    /// Log an info message
    pub fn info(&self, msg: String) {
        let info_msg = format!("[INFO] {}", msg);
        self.log(&info_msg);
    }

    /// Log an error message
    pub fn warning(&self, msg: String) {
        let warning_msg = format!("[WARNING] {}", msg);
        self.log(&warning_msg);
    }

    /// Log an error message
    pub fn error(&self, msg: String) {
        let error_msg = format!("[ERROR] {}: {}", Utc::now(), msg);
        self.log(&error_msg);
    }

    /// Internal function to log an arbitrary message
    fn log(&self, msg: &str) {
        if self.log_to_stdout {
            println!("{}", msg);
        }
        if self.log_to_file {}
    }
}
