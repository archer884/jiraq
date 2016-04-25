#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate clap;
extern crate hyper;
extern crate regex;
extern crate rpassword;
extern crate rustc_serialize;
extern crate serde_json;
extern crate serde;
extern crate toml;

use hyper::Client;
use hyper::header::{ Authorization, Basic };

mod command;
mod config;
mod json;
mod reports;

use json::StorySummary;
use reports::Report;

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
        "existing" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::existing(month)),
        },

        "created" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::created(month)),
        },

        "closed" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::closed(month))
        },

        "remaining" => match config.params().and_then(|p| p.parse().ok()) {
            None => panic!("valid month param not provided"),
            Some(month) => print_report(&client, &header, reports::remaining(month))
        },

        _ => println!("unknown report"),
    }
}

fn print_report(client: &Client, auth: &Authorization<Basic>, report: Report) {
    let report = report.run(client, auth);

    for row in report.chunks(2) {
        println!("{},{}",
            row[0].to::<StorySummary>().unwrap().total(),
            row[1].to::<StorySummary>().unwrap().total());
    }
}

#[inline]
fn get_password() -> String {
    println!("Enter password: ");
    rpassword::read_password().unwrap()
}

#[cfg(test)]
mod tests {
    use hyper::status::StatusCode;
    use json::{ JsonResponse, StorySummary};
    use serde_json;

    const JSON_CONTENT: &'static str = r#"{"startAt":0,"maxResults":0,"total":0,"issues":[]}"#;

    #[test]
    fn can_deserialize() {
        assert!(0 == serde_json::from_str::<StorySummary>(JSON_CONTENT).unwrap().total());
    }

    #[test]
    fn can_deserialize_JsonResponse() {
        let json_response = JsonResponse::new(StatusCode::Ok, JSON_CONTENT.to_owned());

        assert!(0 == json_response.to::<StorySummary>().unwrap().total());
    }
}
