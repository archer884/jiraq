extern crate clap;
extern crate hyper;
extern crate regex;
extern crate rpassword;
extern crate rustc_serialize;
extern crate toml;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::{ Authorization, Basic };
use regex::Regex;

mod command;
mod config;
mod reports;

use reports::Report;

#[derive(Debug)]
struct JsonResponse(StatusCode, String);

impl JsonResponse {
    fn body(&self) -> &str {
        &self.1
    }

    fn total(&self) -> i32 {
        let pattern = Regex::new(r#"("total":)(\d+)"#).unwrap(); // surely this is fine >.>
        match pattern.captures(self.body()) {
            None => panic!("bad response: {}", self.body()),
            Some(captures) => captures.iter().nth(2).and_then(|capture|
                capture.and_then(|n| n.parse().ok())
            ).unwrap_or_else(|| panic!("failed to parse: {}", self.body()))
        }
    }
}

fn main() {
    let config = match config::read_config(command::read_command()) {
        Err(e) => panic!("bad config: {:?}", e),
        Ok(config) => config,
    };

    let client = Client::new();
    let header = Authorization(Basic {
        username: config.username().into(),
        password: Some(config.password().unwrap_or_else(|| get_password())),
    });

    match &config.report()[..] {
        "existing-stories" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::stories::existing(month)),
        },
        "created-stories" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::stories::created(month)),
        },
        "closed-stories" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::stories::closed(month))
        },
        "remaining-stories" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::stories::remaining(month))
        },
        _ => println!("unknown report"),
    }
}

fn print_report(client: &Client, auth: &Authorization<Basic>, report: Report) {
    let report = report.run(client, auth);
    for row in report.chunks(2) {
        println!("{},{}", row[0].total(), row[1].total());
    }
}

#[inline]
fn get_password() -> String {
    println!("Enter password: ");
    rpassword::read_password().unwrap()
}
