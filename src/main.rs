extern crate hyper;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::{ Authorization, Basic };

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
    // This is a terrible way to get someone's password and I mostly did it this way for now just
    // to keep my own password out of the repository.
    let (username, password) = read_username_and_password(std::env::args());

    let client = Client::new();
    let header = Authorization(Basic {
        username: username.into(),
        password: Some(password.into())
    });

    print_opening_balances(&client, &header);
    print_stories_added(&client, &header);
    print_stories_completed(&client, &header);
    print_closing_balance(&client, &header);
}

fn print_opening_balances(client: &Client, auth: &Authorization<Basic>) {
    println!("Opening balances: ");
    print_report("Current Month", &reports::opening_balance_current_month(), client, auth);
    print_report("Prior One", &reports::opening_balance_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::opening_balance_prior_month_two(), client, auth);
}

fn print_stories_added(client: &Client, auth: &Authorization<Basic>) {
    println!("Stories added: ");
    print_report("Current Month", &reports::created_current_month(), client, auth);
    print_report("Prior One", &reports::created_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::created_prior_month_two(), client, auth);
}

fn print_stories_completed(client: &Client, auth: &Authorization<Basic>) {
    println!("Stories completed: ");
    print_report("Current Month", &reports::closed_current_month(), client, auth);
    print_report("Prior One", &reports::closed_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::closed_prior_month_two(), client, auth);
}

fn print_closing_balance(client: &Client, auth: &Authorization<Basic>) {
    println!("Closing balances: ");
    print_report("Current Month", &reports::closing_balance_current_month(), client, auth);
    print_report("Prior One", &reports::closing_balance_prior_month_one(), client, auth);
    print_report("Prior Two", &reports::closing_balance_prior_month_two(), client, auth);
}

fn print_report(message: &str, report: &Report, client: &Client, auth: &Authorization<Basic>) {
    println!("{}", message);
    for result in &report.run(client, auth) {
        println!("{:?}: {}", result.status(), result.body());
    }
}

fn read_username_and_password<I: Iterator<Item = String>>(args: I) -> (String, String) {
    let args: Vec<_> = args.skip(1).take(2).collect();

    (args[0].to_owned(), args[1].to_owned())
}
