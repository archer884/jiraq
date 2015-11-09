extern crate clap;
extern crate hyper;
extern crate rpassword;
extern crate rustc_serialize;
extern crate toml;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::{ Authorization, Basic };

mod command;
mod config;
mod reports;

use reports::Report;

struct JsonResponse(StatusCode, String);

impl JsonResponse {
    fn status(&self) -> StatusCode {
        self.0
    }

    fn body(&self) -> &str {
        &self.1
    }
}

fn main() {
    let config = match config::read_config(&command::read_command()) {
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        },
        Ok(config) => config,
    };

    let client = Client::new();
    let header = Authorization(Basic {
        username: config.username().into(),
        password: Some(config.password().unwrap_or_else(|| get_password())),
    });

    match &config.report()[..] {
        "stories" => {
            story_print_opening_balances(&client, &header);
            story_print_stories_added(&client, &header);
            story_print_stories_completed(&client, &header);
            story_print_closing_balance(&client, &header);
        },
        "bugs" => {
            bug_print_opening_balances(&client, &header);
            bug_print_bugs_added(&client, &header);
            bug_print_bugs_completed(&client, &header);
            bug_print_closing_balance(&client, &header);
        }
        _ => println!("unknown report"),
    }
}

#[inline]
fn get_password() -> String {
    println!("Enter password: ");
    rpassword::read_password().unwrap()
}

fn story_print_opening_balances(client: &Client, auth: &Authorization<Basic>) {
    println!("Opening balances: ");
    print_report("Current Month", &reports::stories::opening_balance_current_month(), client, auth);
    print_report("Prior One", &reports::stories::opening_balance_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::stories::opening_balance_prior_month_two(), client, auth);
}

fn story_print_stories_added(client: &Client, auth: &Authorization<Basic>) {
    println!("Stories added: ");
    print_report("Current Month", &reports::stories::created_current_month(), client, auth);
    print_report("Prior One", &reports::stories::created_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::stories::created_prior_month_two(), client, auth);
}

fn story_print_stories_completed(client: &Client, auth: &Authorization<Basic>) {
    println!("Stories completed: ");
    print_report("Current Month", &reports::stories::closed_current_month(), client, auth);
    print_report("Prior One", &reports::stories::closed_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::stories::closed_prior_month_two(), client, auth);
}

fn story_print_closing_balance(client: &Client, auth: &Authorization<Basic>) {
    println!("Closing balances: ");
    print_report("Current Month", &reports::stories::closing_balance_current_month(), client, auth);
    print_report("Prior One", &reports::stories::closing_balance_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::stories::closing_balance_prior_month_two(), client, auth);
}

fn bug_print_opening_balances(client: &Client, auth: &Authorization<Basic>) {
    println!("Opening balances: ");
    print_report("Current Month", &reports::bugs::opening_balance_current_month(), client, auth);
    print_report("Prior One", &reports::bugs::opening_balance_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::bugs::opening_balance_prior_month_two(), client, auth);
}

fn bug_print_bugs_added(client: &Client, auth: &Authorization<Basic>) {
    println!("bugs added: ");
    print_report("Current Month", &reports::bugs::created_current_month(), client, auth);
    print_report("Prior One", &reports::bugs::created_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::bugs::created_prior_month_two(), client, auth);
}

fn bug_print_bugs_completed(client: &Client, auth: &Authorization<Basic>) {
    println!("bugs completed: ");
    print_report("Current Month", &reports::bugs::closed_current_month(), client, auth);
    print_report("Prior One", &reports::bugs::closed_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::bugs::closed_prior_month_two(), client, auth);
}

fn bug_print_closing_balance(client: &Client, auth: &Authorization<Basic>) {
    println!("Closing balances: ");
    print_report("Current Month", &reports::bugs::closing_balance_current_month(), client, auth);
    print_report("Prior One", &reports::bugs::closing_balance_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::bugs::closing_balance_prior_month_two(), client, auth);
}

fn print_report(message: &str, report: &Report, client: &Client, auth: &Authorization<Basic>) {
    println!("{}", message);
    for result in &report.run(client, auth) {
        println!("{:?}: {}", result.status(), result.body());
    }
}
