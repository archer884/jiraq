use std::fmt;
use std::io::Read;

use hyper::Client;
use hyper::client::Response;
use hyper::header::{ Authorization, Basic };
use json::JsonResponse;

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
                JsonResponse::new(res.status, body)
        }).collect()
    }
}

fn read_response(res: &mut Response) -> String {
    let mut buf = String::new();
    res.read_to_string(&mut buf).ok();

    buf
}

#[derive(Debug, Copy, Clone)]
enum Priority {
    High,
    Medium,
    Low,
}

static PRIORITIES: [Priority; 3] = [
    Priority::High,
    Priority::Medium,
    Priority::Low,
];

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Priority::High => write!(f, "high"),
            &Priority::Medium => write!(f, "medium"),
            &Priority::Low => write!(f, "low"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Site {
    Mss,
    Jpc,
}

static SITES: [Site; 2] = [ Site::Jpc, Site::Mss ];

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Site::Mss => write!(f, "mss"),
            &Site::Jpc => write!(f, "jpc"),
        }
    }
}

pub fn existing(month: i32) -> Report {
    Report(PRIORITIES.iter().flat_map(|&priority|
        SITES.iter().map(move |&site| format_existing_story_query(month, priority, site))
    ).collect())
}

fn format_existing_story_query(month: i32, priority: Priority, label: Site) -> String {
    format!("jql=project=webops and issuetype=story and created <= startofmonth({month}) and priority={priority} and (resolved >= startofmonth({month}) or resolved = null) and labels = {label}",
        month = 0 - month,
        priority = priority,
        label = label
    )
}

pub fn created(month: i32) -> Report {
    Report(PRIORITIES.iter().flat_map(|&priority|
        SITES.iter().map(move |&site| format_created_story_query(month, priority, site))
    ).collect())
}

fn format_created_story_query(month: i32, priority: Priority, label: Site) -> String {
    format!("jql=project = WEBOPS AND issuetype = Story AND priority = {priority} AND created >= startOfMonth({month}) AND created <= endOfMonth({month}) AND resolved = NULL AND labels = {label}",
        month = 0 - month,
        priority = priority,
        label = label
    )
}

pub fn closed(month: i32) -> Report {
    Report(PRIORITIES.iter().flat_map(|&priority|
        SITES.iter().map(move |&site| format_closed_story_query(month, priority, site))
    ).collect())
}

fn format_closed_story_query(month: i32, priority: Priority, label: Site) -> String {
    format!("jql=project = WEBOPS AND issuetype = Story AND priority = {priority} AND resolved >= startOfMonth({month}) AND resolved <= endOfMonth({month}) AND  labels = {label}",
        month = 0 - month,
        priority = priority,
        label = label
    )
}

// Closing balances
pub fn remaining(month: i32) -> Report {
    Report(PRIORITIES.iter().flat_map(|&priority|
        SITES.iter().map(move |&site| format_remaining_story_query(month, priority, site))
    ).collect())
}

fn format_remaining_story_query(month: i32, priority: Priority, label: Site) -> String {
    format!("jql=project=webops and issuetype=story and created <= endofmonth({month}) and priority={priority} and (resolved = null or resolved >= endofmonth({month})) and labels = {label}",
        month = 0 - month,
        priority = priority,
        label = label
    )
}
