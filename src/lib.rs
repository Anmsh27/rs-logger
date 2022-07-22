use colored::*;
use std::fs;
use std::io;
use std::io::Write;
use chrono;

/// LoggingLevel enum
/// 1 is for logging everything
/// 2 is for everything except debug
/// 3 is for everything excpet debug + info
/// 4 is for everything only errors
#[derive(Debug)]
pub enum LoggingLevel {
    LevelOne, // everything
    LevelTwo, // no debug
    LevelThree, // no info
    LevelFour, // no warning
}

/// Logger
/// # Examples
/// 
/// ```
/// let logger = Logger::default().unwrap();
/// logger.set_level(LoggingLevel::LevelThree);
/// 
/// logger.info("It works!");
/// ```

#[derive(Debug)]
pub struct Logger {
    file: fs::File,
    level: LoggingLevel
}

impl Default for Logger {
    fn default() -> Self {
        let filename = String::from("logger.log");
        let file = open(&filename).unwrap_or_else(|error| {
            println!("{}", format!("Logger Error: {}", error.to_string()).red());
            panic!("");
        });
        Self {
            file,
            level: LoggingLevel::LevelOne
        }
    }
}

impl Logger {
    pub fn new<T>(filename: T, level: LoggingLevel) -> Result<Self, String>
    where
        T: Into<String>
    {
        use LoggingLevel::*;
        let logging_level: LoggingLevel;

        match level {
            LevelOne => {
                logging_level = LevelOne;
            },
            LevelTwo => {
                logging_level = LevelTwo;
            },
            LevelThree => {
                logging_level = LevelThree;
            },
            LevelFour => {
                logging_level = LevelFour;
            }
        }

        let filename = filename.into();
        let file = open(&filename)?;
        Ok(Self {
            file,
            level: logging_level
        })
    }

    pub fn set_level(&mut self, level: LoggingLevel) {
        
        use LoggingLevel::*;

        match level {
            LevelOne => {
                self.level = LevelOne;
            },
            LevelTwo => {
                self.level = LevelTwo;
            },
            LevelThree => {
                self.level = LevelThree;
            },
            LevelFour => {
                self.level = LevelFour;
            }
        }
    }

    pub fn error<T>(&mut self, msg: T)
    where
        T: Into<String>
    {
        let msg = msg.into();

        let formatted = format!("ERROR: {}: '{}'", file!(), msg);
        println!("{}", &formatted.bright_red());

        let date = chrono::Utc::now().date();
        let date_string = date.to_string().replace("UTC", "");
        let time = chrono::Utc::now()
            .time()
            .format("%H:%M:%S")
            .to_string();

        let file_formatted = format!("{} {} ERROR: {}: '{}'\n",
            date_string,
            time,
            file!(),
            msg
        );

        write_file(&mut self.file, &file_formatted).expect("Couldn't write to file");
    }

    pub fn info<T>(&mut self, msg: T)
    where
        T: Into<String>
    {
        use LoggingLevel::*;

        let msg = msg.into();

        let formatted = format!("INFO: {}", msg);
        match self.level {
            LevelOne => {
                println!("{}", &formatted.green());
            },
            _ => {}
        };

        let date = chrono::Utc::now().date();
        let date_string = date.to_string().replace("UTC", "");
        let time = chrono::Utc::now()
            .time()
            .format("%H:%M:%S")
            .to_string();

        let file_formatted = format!("{} {} INFO: '{}'\n",
            date_string,
            time,
            msg
        );

        write_file(&mut self.file, &file_formatted).expect("Couldn't write to file");
    }

    pub fn warning<T>(&mut self, msg: T)
    where
        T: Into<String>
    {
        use LoggingLevel::*;

        let msg = msg.into();

        let formatted = format!("WARNING: {}: '{}'", file!(), msg);
        match self.level {
            LevelOne | LevelTwo => {
                println!("{}", &formatted.bright_yellow());
            },
            _ => {}
        };

        let date = chrono::Utc::now().date();
        let date_string = date.to_string().replace("UTC", "");
        let time = chrono::Utc::now()
            .time()
            .format("%H:%M:%S")
            .to_string();

        let file_formatted = format!("{} {} WARNING: {}: '{}'\n",
            date_string,
            time,
            file!(),
            msg
        );

        write_file(&mut self.file, &file_formatted).expect("Couldn't write to file");
    }

    pub fn debug<T>(&mut self, msg: T)
    where
        T: Into<String>
    {
        use LoggingLevel::*;

        let msg = msg.into();

        let formatted = format!("DEBUG: {}", msg);
        match self.level {
            LevelOne => {
                println!("{}", &formatted.blue());
            },
            _ => {}
        };

        let date = chrono::Utc::now().date();
        let date_string = date.to_string().replace("UTC", "");
        let time = chrono::Utc::now()
            .time()
            .format("%H:%M:%S")
            .to_string();

        let file_formatted = format!("{} {} DEBUG: '{}'\n",
            date_string,
            time,
            msg
        );

        write_file(&mut self.file, &file_formatted).expect("Couldn't write to file");
    }
}

fn write_file<T: Into<String>>(file: &mut fs::File, msg: T) -> Result<(), String> {
    let msg: String = msg.into();

    match file.write_all(&mut msg.as_bytes()) {
        Ok(i) => i,
        Err(error) => return Err(error.to_string())
    };

    Ok(())
}

fn open<T: Into<String>>(filename: T) -> Result<fs::File, String> {
    let filename = filename.into();
    let file = match fs::OpenOptions::new()
        .write(true)
        .open(&filename) {
            Ok(i) => i,
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::NotFound => {
                        let file = match fs::File::create(&filename) {
                            Ok(i) => i,
                            Err(error) => return Err(error.to_string())
                        };
                        file
                    },
                    _ => return Err(error.to_string())
                }
            }
        };
    Ok(file)
}
