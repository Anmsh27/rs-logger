use chrono;
use colored::*;
use json;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};

mod macros;

/// LoggingLevel enum
///
/// pass it into a logger
#[derive(Debug)]
pub enum LoggingLevel {
    LevelOne,   // everything
    LevelTwo,   // no debug
    LevelThree, // no info
    LevelFour,  // no warning
    LevelFive,  // no error
}

/// Config Struct
/// 
/// Pass into Logger while creating a new logger
/// ```
/// let config = Config::new()
///     .filename("logs.json")
///     .json(true)
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    json: bool,                   // if the output should be json or not
    filename: String,             // filename
    json_object: json::JsonValue, // for writing json to the file
}

impl Config {
    // new config
    pub fn new() -> Self {
        let filename = String::from("logs.log"); // default filename
        Self {
            json: false, // json false by default
            filename,
            json_object: json::object! { // initialize json object with "logs" array
                logs: json_array![

                ],
            },
        }
    }

    // change filename
    pub fn filename<T: Into<String>>(&mut self, filename: T) -> Self {
        self.filename = filename.into();
        self.update_filename();
        self.clone()
    }

    // change json value
    pub fn json(&mut self, json: bool) -> Self {
        self.json = json;
        self.update_filename();
        self.clone()
    }

    // updates filename
    fn update_filename(&mut self) {
        if self.json {
            if !self.filename.ends_with(".json") {
                self.filename += ".json"
            };
        }
    }

    // returns reference to filename
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}

/// Logger
/// 
/// Logger::default() returns default logger
/// 
/// Logger::new() takes in the logging level and config object
///
/// ```
/// let config = Config::new()
///     .filename("Logs.json")
///     .json(true)
/// let logger = Logger::new(LoggingLevel::LevelOne, config);
/// logger.debug("It works");
/// ```
#[derive(Debug)]
pub struct Logger {
    file: fs::File,      // the file to be written to
    level: LoggingLevel, // the logging level
    config: Config,      // the config
}

impl Default for Logger {
    // default logger with conifg set to filename "logs" and json true
    fn default() -> Self {
        let config = Config::new().filename("logs").json(true);

        let file = open(config.get_filename()).unwrap_or_else(|error| {
            println!("{}", format!("Logger Error: {}", error.to_string()).red());
            panic!("");
        });

        Self {
            file,
            level: LoggingLevel::LevelOne,
            config,
        }
    }
}

impl Logger {
    // new logger
    pub fn new(level: LoggingLevel, config: Config) -> Result<Self, String> {   
        let logging_level = level;

        let config = config;
        let file = open(config.get_filename())?;

        Ok(Self {
            file,
            level: logging_level,
            config: config,
        })
    }

    // set the level
    pub fn set_level(&mut self, level: LoggingLevel) {
        self.level = level;
    }

    // critical
    // ignores logging level
    pub fn critical<T>(&mut self, msg: T)
    where
        T: Into<String>,
    {
        let mut msg = msg.into();

        let mut formatters: HashMap<String, String> = HashMap::new();

        let (date_string, time) = get_date_time();

        formatters.insert("%T".to_string(), String::from(&time));
        formatters.insert("%D".to_string(), String::from(&date_string));

        for formatter in &formatters {
            if msg.find(formatter.0).is_some() {
                msg = msg.replace(formatter.0, formatter.1);
            }
        }

        let formatted = format!("CRITICAL: {}", msg);
        println!("{}", &formatted.on_red().red());

        let file_formatted = format!("{} {} CRITICAL: {}\n", date_string, time, msg);

        match self.config.json {
            true => {
                let data_to_be_written = json::object! {
                    date: string!(date_string),
                    time: string!(time),
                    message: string!(msg),
                    type: string!("CRITICAL")
                };

                self.config.json_object["logs"]
                    .push(data_to_be_written)
                    .expect("Couldn't parse json");
            }
            false => write_file(&mut self.file, &file_formatted).expect("Couldn't write to file"),
        }
    }

    // error
    // only works till level four
    // is ignored by level five
    pub fn error<T>(&mut self, msg: T)
    where
        T: Into<String>,
    {
        use LoggingLevel::*;

        let mut msg = msg.into();

        let mut formatters: HashMap<String, String> = HashMap::new();

        let (date_string, time) = get_date_time();

        formatters.insert("%T".to_string(), String::from(&time));
        formatters.insert("%D".to_string(), String::from(&date_string));

        for formatter in &formatters {
            if msg.find(formatter.0).is_some() {
                msg = msg.replace(formatter.0, formatter.1);
            }
        }

        let formatted = format!("ERROR: {}", msg);
        match self.level {
            LevelOne | LevelTwo | LevelThree | LevelFour => println!("{}", &formatted.bright_red()),
            _ => {}
        }

        let file_formatted = format!("{} {} ERROR: {}\n", date_string, time, msg);

        match self.config.json {
            true => {
                let data_to_be_written = json::object! {
                    date: string!(date_string),
                    time: string!(time),
                    message: string!(msg),
                    type: string!("ERROR")
                };

                self.config.json_object["logs"]
                    .push(data_to_be_written)
                    .expect("Couldn't parse json");
            }
            false => write_file(&mut self.file, &file_formatted).expect("Couldn't write to file"),
        }
    }

    // info
    // only works till level two
    // ignored level three and onwards
    pub fn info<T>(&mut self, msg: T)
    where
        T: Into<String>,
    {
        use LoggingLevel::*;

        let mut msg = msg.into();

        let mut formatters: HashMap<String, String> = HashMap::new();

        let (date_string, time) = get_date_time();

        formatters.insert("%T".to_string(), String::from(&time));
        formatters.insert("%D".to_string(), String::from(&date_string));

        for formatter in &formatters {
            if msg.find(formatter.0).is_some() {
                msg = msg.replace(formatter.0, formatter.1);
            }
        }

        let formatted = format!("INFO: {}", msg);
        match self.level {
            LevelOne | LevelTwo => {
                println!("{}", &formatted.green());
            }
            _ => {}
        };

        let file_formatted = format!("{} {} INFO: {}\n", date_string, time, msg);

        match self.config.json {
            true => {
                let data_to_be_written = json::object! {
                    date: string!(date_string),
                    time: string!(time),
                    message: string!(msg),
                    type: string!("INFO")
                };

                self.config.json_object["logs"]
                    .push(data_to_be_written)
                    .expect("Couldn't parse json");
            }
            false => write_file(&mut self.file, &file_formatted).expect("Couldn't write to file"),
        }
    }

    // warning
    // only works until level three
    // ignored by level four onwards
    pub fn warning<T>(&mut self, msg: T)
    where
        T: Into<String>,
    {
        use LoggingLevel::*;

        let mut msg = msg.into();

        let mut formatters: HashMap<String, String> = HashMap::new();

        let (date_string, time) = get_date_time();

        formatters.insert("%T".to_string(), String::from(&time));
        formatters.insert("%D".to_string(), String::from(&date_string));

        for formatter in &formatters {
            if msg.find(formatter.0).is_some() {
                msg = msg.replace(formatter.0, formatter.1);
            }
        }

        let formatted = format!("WARNING: {}", msg);
        match self.level {
            LevelOne | LevelTwo | LevelThree => {
                println!("{}", &formatted.bright_yellow());
            }
            _ => {}
        };

        let file_formatted = format!("{} {} WARNING: {}\n", date_string, time, msg);

        match self.config.json {
            true => {
                let data_to_be_written = json::object! {
                    date: string!(date_string),
                    time: string!(time),
                    message: string!(msg),
                    type: string!("WARNING")
                };

                self.config.json_object["logs"]
                    .push(data_to_be_written)
                    .expect("Couldn't parse json");
            }
            false => write_file(&mut self.file, &file_formatted).expect("Couldn't write to file"),
        }
    }

    // debug
    // only works on level one
    pub fn debug<T>(&mut self, msg: T)
    where
        T: Into<String>,
    {
        use LoggingLevel::*;

        let mut msg = msg.into();

        let mut formatters: HashMap<String, String> = HashMap::new();

        let (date_string, time) = get_date_time();

        formatters.insert("%T".to_string(), String::from(&time));
        formatters.insert("%D".to_string(), String::from(&date_string));

        for formatter in &formatters {
            if msg.find(formatter.0).is_some() {
                msg = msg.replace(formatter.0, formatter.1);
            }
        }

        let formatted = format!("DEBUG: {}", msg);
        match self.level {
            LevelOne => {
                println!("{}", &formatted.blue());
            }
            _ => {}
        };

        let file_formatted = format!("{} {} DEBUG: {}\n", date_string, time, msg);

        match self.config.json {
            true => {
                let data_to_be_written = json::object! {
                    date: string!(date_string),
                    time: string!(time),
                    message: string!(msg),
                    type: string!("DEBUG")
                };

                self.config.json_object["logs"]
                    .push(data_to_be_written)
                    .expect("Couldn't parse json");
            }
            false => write_file(&mut self.file, &file_formatted).expect("Couldn't write to file"),
        }
    }
}

// json is written when the Logger goes out of scope and is dropped
impl Drop for Logger {
    fn drop(&mut self) {
        match self.config.json {
            true => {
                write_file(&mut self.file, self.config.json_object.to_string().trim())
                    .expect("Couldn't write to file");
            }
            false => {}
        }
    }
}

// get currednt UTC date and time
fn get_date_time() -> (String, String) {
    let date = chrono::Utc::now().date();
    let date_string = date.to_string().replace("UTC", "");
    let time = chrono::Utc::now().time().format("%H:%M:%S").to_string();

    (date_string, time)
}

// write to file
fn write_file<T: Into<String>>(file: &mut fs::File, msg: T) -> Result<(), String> {
    let msg: String = msg.into();

    match file.write(&mut msg.as_bytes()) {
        Ok(i) => i,
        Err(error) => return Err(error.to_string()),
    };

    Ok(())
}

// open file
fn open<T: Into<String>>(filename: T) -> Result<fs::File, String> {
    let filename = filename.into();
    let file = match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&filename)
    {
        Ok(i) => i,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                let file = match fs::File::create(&filename) {
                    Ok(i) => i,
                    Err(error) => return Err(error.to_string()),
                };
                file
            }
            _ => return Err(error.to_string()),
        },
    };
    Ok(file)
}
