use chrono::prelude::*;
use std::fmt;
use std::panic::Location;

use mysql::execute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

pub struct Logger {}

impl Logger {
    async fn log_internal(&self, level: LogLevel, location: &Location<'_>, message: &str) {
        let now = Utc::now();
        let line = format!(
            "{}: {} @ {}: line {} - {}",
            now.format("%Y-%m-%d %H:%M:%S"),
            level,
            location.file(),
            location.line(),
            message
        );
        println!("{}", line);

        if level == LogLevel::Warn || level == LogLevel::Error {
            Self::create_table().await;

            let query = format!(
                "INSERT INTO system_logs (level, location, message) VALUES ('{}', '{}: {} ', '{}')",
                level,
                location.file(),
                location.line(),
                message
            );

            match execute(query.as_str(), None).await {
                Ok(_) => println!("Inserted successfully"),
                Err(err) => println!("Error inserting: {}", err),
            }
        }
    }

    async fn create_table() {
        let query = "CREATE TABLE if not exists system_logs (
            id INT AUTO_INCREMENT PRIMARY KEY,
            datetime TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            level VARCHAR(5),
            location VARCHAR(255),
            message TEXT
        );"
        .to_owned();

        match execute(query.as_str(), None).await {
            Ok(_) => println!("Table created successfully"),
            Err(err) => println!("Error creating table: {}", err),
        }
    }

    pub async fn debug(&self, location: &Location<'_>, message: &str) {
        self.log_internal(LogLevel::Debug, location, message).await;
    }

    pub async fn info(&self, location: &Location<'_>, message: &str) {
        self.log_internal(LogLevel::Info, location, message).await;
    }

    pub async fn warn(&self, location: &Location<'_>, message: &str) {
        self.log_internal(LogLevel::Warn, location, message).await;
    }

    pub async fn error(&self, location: &Location<'_>, message: &str) {
        self.log_internal(LogLevel::Error, location, message).await;
    }
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $msg:expr) => {
        $logger.debug(std::panic::Location::caller(), $msg)
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $msg:expr) => {
        $logger.info(std::panic::Location::caller(), $msg)
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $msg:expr) => {
        $logger.warn(std::panic::Location::caller(), $msg)
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $msg:expr) => {
        $logger.error(std::panic::Location::caller(), $msg)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_debug() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");

        let logger = Logger {};
        logger
            .debug(Location::caller(), "This is a debug log")
            .await;
    }

    #[tokio::test]
    async fn test_info() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");

        let logger = Logger {};
        logger.info(Location::caller(), "This is an info log").await;
    }

    #[tokio::test]
    async fn test_warn() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");

        let logger = Logger {};
        logger.warn(Location::caller(), "This is a warn log").await;
    }

    #[tokio::test]
    async fn test_error() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");

        let logger = Logger {};
        logger
            .error(Location::caller(), "This is an error log")
            .await;
    }
}
