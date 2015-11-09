use std::io::Read;

use hyper::Client;
use hyper::client::Response;
use hyper::header::{ Authorization, Basic };

use super::JsonResponse;

pub mod stories;
pub mod bugs;

const URL_BASE: &'static str = "https://magdevelopment.atlassian.net/rest/api/2/search?maxResults=0&";

// I'm representing a report as just a vector of queries
pub struct Report(Vec<String>);

impl Report {
    pub fn run(&self, client: &Client, auth: &Authorization<Basic>) -> Vec<JsonResponse> {
        self.0.iter().map(|jql| {
            let mut res = client.get(&format!("{}{}", URL_BASE, jql))
                .header(auth.clone()) // I have no idea why headers are clone, or why this is not a borrow
                .send()
                .unwrap();

            let body = read_response(&mut res);
            JsonResponse(res.status, body)
        }).collect()
    }
}

fn read_response(res: &mut Response) -> String {
    let mut buf = String::new();
    res.read_to_string(&mut buf).ok();

    buf
}
