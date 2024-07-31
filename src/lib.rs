use std::fs::{File, OpenOptions};
use std::io::Write;
use std::str::FromStr;
use chrono::Local;
use colored::*;

/// Enum representing the type of message to be logged or printed.
pub enum PALMessageType {
    Success,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl PALMessageType {
    fn as_str(&self) -> &str {
        match self {
            PALMessageType::Success => "SUCCESS",
            PALMessageType::Error => "ERROR",
            PALMessageType::Warn => "WARN",
            PALMessageType::Info => "INFO",
            PALMessageType::Debug => "DEBUG",
            PALMessageType::Trace => "TRACE",
        }
    }
}
pub struct PrintAndLog {
    log_to_file: bool,
    log_file_handle: Option<File>,
    log_file_path: String,
    max_file_size: u32, //in bytes, so maximum of 4294.9 MB
    timestamp_format: String,
}

impl PrintAndLog {
    /// Create a new instance of PrintAndLog with default settings.
    pub fn new() -> Self {
        PrintAndLog {
            log_to_file: true,
            log_file_handle: None,
            log_file_path: String::from("application.log"),
            max_file_size: 10000000, //10 MB
            timestamp_format: String::from_str("%d.%m.%Y %H:%M:%S").unwrap(), // https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        }
    }

    //Settings wrapper functions
    pub fn set_log_to_file(&mut self, setting: bool) {
        self.log_to_file = setting;
    }

    pub fn get_log_to_file(&self) -> bool {
        return self.log_to_file;
    }

    pub fn set_log_file_name <T: Into<String>> (&mut self, file_name: T) -> Result<(), &'static str> {
        let file_name_str = file_name.into();
        
        if self.is_valid_filename(&file_name_str) {
            self.log_file_path = file_name_str;
            Ok(())
        }
        else {
            Err("File name is not valid.")
        }
    }

    pub fn get_log_file_name (&self) -> String {
        return self.log_file_path.clone();
    }

    pub fn set_max_file_size (&mut self, new_file_size_in_bytes: u32) -> Result<(), &'static str> {
        
        if new_file_size_in_bytes > 0 {
            self.max_file_size = new_file_size_in_bytes;
            Ok(())
        }
        else {
            Err("File size cannot be 0.")
        }
    }

    pub fn get_max_file_size (&self) -> u32 {
        return self.max_file_size;
    }

    pub fn set_timestamp_format_for_print <T: Into<String>> (&mut self, format: T) -> Result<(), &'static str> {
        let format_string = format.into();
        
        if format_string.is_empty() {
            Err("Invalid timestamp.")
        }
        else {
            self.timestamp_format = format_string;
            Ok(())
        }
    }

    pub fn get_timestamp_format_for_print (&self) -> String {
        return self.timestamp_format.clone();
    }

    //Internal functions
    fn get_timestamp(&self) -> String {
        return Local::now().format(&self.timestamp_format).to_string();
    }

    fn get_log_timestamp(&self) -> String {
        return Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    }

    fn is_valid_filename(&self,file_name: &String) -> bool {
        !file_name.is_empty() && file_name.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-')
    }

    fn open_log_file(&mut self) -> Result<(), &'static str> {
        match OpenOptions::new().create(true).write(true).append(true).open(&self.log_file_path) {
            Ok(file) => {
                self.log_file_handle = Some(file);
                Ok(())
            }
            Err(_) => Err("Unable to create log file"),           
        }
    }

    fn rotate_log_file(&mut self) -> Result<(), &'static str> {
        let new_log_file_path = format!("{}.{}", self.get_log_timestamp(), &self.log_file_path);

        if let Err(_) = std::fs::rename(&self.log_file_path,new_log_file_path) {
            return Err("Can't rename the log file.");
        }

        self.log_file_handle = None;
        return self.open_log_file();
    }

    //Main public functions
    pub fn print <T: AsRef<str>, U:AsRef<str>> (&self,message_title: T, message: U, message_type: &PALMessageType) {
        let message_title: &str = message_title.as_ref();
        let message: &str = message.as_ref();

        if message_title.is_empty() {
            println!("[ {} ] {}",self.get_timestamp(),message);
        }
        else {
            let formatted_message_title: ColoredString =  match message_type {
                PALMessageType::Success => message_title.green().bold(),
                PALMessageType::Info => message_title.white().bold(),
                PALMessageType::Debug => message_title.blue().bold(),
                PALMessageType::Trace => message_title.magenta().bold(),
                PALMessageType::Warn => message_title.yellow().bold(),
                PALMessageType::Error => message_title.red().bold(),
            };

            println!("[ {} ] {} {}",self.get_timestamp(),formatted_message_title,message);
        }
    }

    pub fn log <T: AsRef<str>, U:AsRef<str>> (&mut self, message_title: T, message: U, message_type: &PALMessageType) {
        let message_title: &str = message_title.as_ref();
        let message: &str = message.as_ref();

        if self.log_to_file {

            let formatted_message = format!(
                "{} {} {}: {}\n",
                self.get_log_timestamp(),
                message_type.as_str(),
                message_title,
                message
            );

            if self.log_file_handle.is_none() {
                if let Err(e) = self.open_log_file() {
                    self.print("Error", format!("Cannot open the file:{}",e), &PALMessageType::Error);
                    return;
                }
            }

            if let Some(ref mut file) = self.log_file_handle {
                let should_rotate = if let Ok(metadata) = file.metadata() {
                    metadata.len() > self.max_file_size.into()
                }
                else {
                    false
                };

                if should_rotate {
                    if let Err(e) = self.rotate_log_file() {
                        self.print("Error",e,&PALMessageType::Error);
                        return;
                    }
                }
            }


            match self.log_file_handle.as_ref().expect("Can't write to file.").write_all(formatted_message.as_bytes()) {
                Ok(())=> (),
                Err(e) => {
                    self.print("Error", &format!("Unable to write into log file: {}", e), &PALMessageType::Error);
                    return;
                }
            }
        }
    }

    pub fn print_and_log <T: AsRef<str>, U:AsRef<str>> (&mut self,message_title: T, message: U, message_type: &PALMessageType) {
        let title_str = message_title.as_ref();
        let message_str = message.as_ref();
        
        self.print(&title_str, &message_str, message_type);
        self.log(&title_str, &message_str, message_type);
    }
}

impl Drop for PrintAndLog {
    fn drop (&mut self) {
        if let Some(ref mut file) = self.log_file_handle {
            if let Err(e) = file.flush() {
                self.print("Error",format!("Can't flush log file: {}",e),&PALMessageType::Error);
            }
        }
    }
}