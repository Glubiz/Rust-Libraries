use chrono::prelude::*;
use std::fmt;
use std::panic::Location;
use tokio::task;

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
    async fn log_internal(level: LogLevel, location: &Location<'_>, message: &str) {
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

            let _ = task::spawn(async move {
                match execute(query.as_str(), None).await {
                    Ok(_) => println!("Inserted successfully"),
                    Err(err) => println!("Error inserting: {}", err),
                }
            })
            .await;
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

        let _ = task::spawn(async move {
            match execute(query.as_str(), None).await {
                Ok(_) => println!("Table created successfully"),
                Err(err) => println!("Error creating table: {}", err),
            }
        })
        .await;
    }

    pub async fn log(level: LogLevel, location: &Location<'_>, message: &str) {
        Self::log_internal(level, location, message).await;
    }
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        tokio::spawn(async move {
            let location = std::panic::Location::caller();
            Logger::log(LogLevel::Debug, &location, $msg).await;
        })
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        tokio::spawn(async move {
            let location = std::panic::Location::caller();
            Logger::log(LogLevel::Info, &location, $msg).await;
        })
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        tokio::spawn(async move {
            let location = std::panic::Location::caller();
            Logger::log(LogLevel::Warn, &location, $msg).await;
        })
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        tokio::spawn(async move {
            let location = std::panic::Location::caller();
            Logger::log(LogLevel::Error, &location, $msg).await;
        })
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

        let _ = debug!("This is a debug log").await;
    }

    #[tokio::test]
    async fn test_debug_format() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");
        let message = "This is a debug log";
        let _ = debug!(format!("{}", message).as_str()).await;
    }

    #[tokio::test]
    async fn test_info() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");

        let _ = info!("This is a info log").await;
    }

    #[tokio::test]
    async fn test_info_format() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");
        let message = "This is a info log";
        let _ = info!(format!("{}", message).as_str()).await;
    }

    #[tokio::test]
    async fn test_warn() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");

        let _ = warn!("This is a warn log").await;
    }

    #[tokio::test]
    async fn test_warn_format() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");
        let message = "This is a warn log";
        let _ = warn!(format!("{}", message).as_str()).await;
    }

    #[tokio::test]
    async fn test_error() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");

        let _ = error!("This is an error log").await;
    }

    #[tokio::test]
    async fn test_error_format() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "mysql://root:password@localhost/test");
        let message = "This is a error log";
        let _ = error!(format!("{}", message).as_str()).await;
    }
}
