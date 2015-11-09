use command::Command;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{ Path, PathBuf };
use toml;
use toml::Parser;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
pub enum ConfigError {
    NotFound { path: String },
    Invalid { content: String },
}

impl ConfigError {
    fn not_found(path: &Path) -> ConfigError {
        ConfigError::NotFound { path: path.to_str().unwrap_or("unrepresentable path").into() }
    }

    fn invalid(content: String) -> ConfigError {
        ConfigError::Invalid { content: content.into() }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Config {
    username: String,
    password: Option<String>,
    report: Option<String>,
}

impl Config {
    pub fn username(&self) -> String {
        self.username.to_owned()
    }

    pub fn password(&self) -> Option<String> {
        self.password.clone()
    }

    pub fn report(&self) -> String {
        self.report.clone().unwrap()
    }
}

pub fn read_config(command: &Command) -> Result<Config, ConfigError> {
    let path = config_path();
    let content = try!(File::open(&path).map_err(|_| ConfigError::not_found(&path)).map(to_string));
    let toml = try!(Parser::new(&content).parse().ok_or(ConfigError::invalid(content.clone())));

    toml::decode::<Config>(toml["config"].clone()).ok_or(ConfigError::invalid(content.clone()))
        .map(|mut config| {
            // Add password from stdin if possible -- avoids asking user for it
            if command.with_password {
                let handle = io::stdin();
                let mut reader = handle.lock();
                let mut buffer = String::new();

                reader.read_line(&mut buffer).ok();

                config.password = Some(buffer.trim().to_owned());
            }

            // Add reports
            config.report = Some(command.report.to_owned());

            config
        })
}

fn config_path() -> PathBuf {
    env::home_dir().map(|mut path| {
        path.push(".jiraq.toml");
        path
    }).unwrap()
}

fn to_string<T: Read>(mut stream: T) -> String {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).ok();

    buffer
}
