// These features mean that this code is no longer buildable on anything other than nightly. I
// don't personally see this as a big deal because I don't use anything else, but on the up side we
// could salvage this by implementing the `Deserialize` trait by hand. So far, the struct has only
// one field, and that wouldn't be a big deal. It's just faster this way because I've never done it
// the other way. Except, you know... the dependency on `aster` takes around a month to compile.
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
use hyper::status::StatusCode;

mod command;
mod config;
mod json;
mod reports;

use json::StorySummary;
use reports::Report;

#[derive(Debug)]
struct JsonResponse(StatusCode, String);

impl JsonResponse {
    fn body(&self) -> &str {
        &self.1
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

    // here we kind of assume the user wants story reports; this tool is getting more specialized
    let story_summaries: Vec<_> = report.iter().map(|response|
        serde_json::from_str::<StorySummary>(response.body()).unwrap()
    ).collect();

    for row in story_summaries.chunks(2) {
        println!("{},{}", row[0].total(), row[1].total());
    }
}

#[inline]
fn get_password() -> String {
    println!("Enter password: ");
    rpassword::read_password().unwrap()
}

#[cfg(test)]
mod tests {
    use json::StorySummary;
    use serde_json;

    #[test]
    fn can_deserialize() {
        let json = r#"{"startAt":0,"maxResults":0,"total":0,"issues":[]}"#;

        assert!(0 == serde_json::from_str::<StorySummary>(json).unwrap().total());
    }
}
