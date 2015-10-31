use std::io::Read;

use hyper::Client;
use hyper::client::Response;
use hyper::header::{ Authorization, Basic };
use super::JsonResponse;

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

// opening balances

pub fn opening_balance_current_month() -> Report {
    Report(vec![
        "jql=project=webops and issuetype=story and created <= startofmonth() and priority=high and (resolved >= startofmonth() or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth() and priority=high and (resolved >= startofmonth() or resolved = null) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth() and priority=medium and (resolved >= startofmonth() or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth() and priority=medium and (resolved >= startofmonth() or resolved = null) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth() and priority=low and (resolved >= startofmonth() or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth() and priority=low and (resolved >= startofmonth() or resolved = null) and labels = mss".to_owned(),
    ])
}

pub fn opening_balance_prior_month_one() -> Report {
    Report(vec![
        "jql=project=webops and issuetype=story and created <= startofmonth(-1) and priority=high and (resolved >= startofmonth(-1) or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-1) and priority=high and (resolved >= startofmonth(-1) or resolved = null) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-1) and priority=medium and (resolved >= startofmonth(-1) or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-1) and priority=medium and (resolved >= startofmonth(-1) or resolved = null) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-1) and priority=low and (resolved >= startofmonth(-1) or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-1) and priority=low and (resolved >= startofmonth(-1) or resolved = null) and labels = mss".to_owned(),
    ])
}

pub fn opening_balance_prior_month_two() -> Report {
    Report(vec![
        "jql=project=webops and issuetype=story and created <= startofmonth(-2) and priority=high and (resolved >= startofmonth(-2) or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-2) and priority=high and (resolved >= startofmonth(-2) or resolved = null) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-2) and priority=medium and (resolved >= startofmonth(-2) or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-2) and priority=medium and (resolved >= startofmonth(-2) or resolved = null) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-2) and priority=low and (resolved >= startofmonth(-2) or resolved = null) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= startofmonth(-2) and priority=low and (resolved >= startofmonth(-2) or resolved = null) and labels = mss".to_owned(),
    ])
}


// Created in month

pub fn created_current_month() -> Report {
    Report(vec![
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND created >= startOfMonth() AND created <= endOfMonth() AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND created >= startOfMonth() AND created <= endOfMonth() AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND created >= startOfMonth() AND created <= endOfMonth() AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND created >= startOfMonth() AND created <= endOfMonth() AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND created >= startOfMonth() AND created <= endOfMonth() AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND created >= startOfMonth() AND created <= endOfMonth() AND resolved = NULL AND labels = MSS".to_owned(),
    ])
}

pub fn created_prior_month_one() -> Report {
    Report(vec![
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND created >= startOfMonth(-1) AND created <= endOfMonth(-1) AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND created >= startOfMonth(-1) AND created <= endOfMonth(-1) AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND created >= startOfMonth(-1) AND created <= endOfMonth(-1) AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND created >= startOfMonth(-1) AND created <= endOfMonth(-1) AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND created >= startOfMonth(-1) AND created <= endOfMonth(-1) AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND created >= startOfMonth(-1) AND created <= endOfMonth(-1) AND resolved = NULL AND labels = MSS".to_owned(),
    ])
}

pub fn created_prior_month_two() -> Report {
    Report(vec![
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND created >= startOfMonth(-2) AND created <= endOfMonth(-2) AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND created >= startOfMonth(-2) AND created <= endOfMonth(-2) AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND created >= startOfMonth(-2) AND created <= endOfMonth(-2) AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND created >= startOfMonth(-2) AND created <= endOfMonth(-2) AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND created >= startOfMonth(-2) AND created <= endOfMonth(-2) AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND created >= startOfMonth(-2) AND created <= endOfMonth(-2) AND resolved = NULL AND labels = MSS".to_owned(),
    ])
}

// Closed in month

pub fn closed_current_month() -> Report {
    Report(vec![
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved >= startOfMonth() AND resolved <= endOfMonth() AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved >= startOfMonth() AND resolved <= endOfMonth() AND  labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved >= startOfMonth() AND resolved <= endOfMonth() AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved >= startOfMonth() AND resolved <= endOfMonth() AND  labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved >= startOfMonth() AND resolved <= endOfMonth() AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved >= startOfMonth() AND resolved <= endOfMonth() AND  labels = MSS".to_owned(),
    ])
}

pub fn closed_prior_month_one() -> Report {
    Report(vec![
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved >= startOfMonth(-1) AND resolved <= endOfMonth(-1) AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved >= startOfMonth(-1) AND resolved <= endOfMonth(-1) AND  labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved >= startOfMonth(-1) AND resolved <= endOfMonth(-1) AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved >= startOfMonth(-1) AND resolved <= endOfMonth(-1) AND  labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved >= startOfMonth(-1) AND resolved <= endOfMonth(-1) AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved >= startOfMonth(-1) AND resolved <= endOfMonth(-1) AND  labels = MSS".to_owned(),
    ])
}

pub fn closed_prior_month_two() -> Report {
    Report(vec![
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved >= startOfMonth(-2) AND resolved <= endOfMonth(-2) AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved >= startOfMonth(-2) AND resolved <= endOfMonth(-2) AND  labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved >= startOfMonth(-2) AND resolved <= endOfMonth(-2) AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved >= startOfMonth(-2) AND resolved <= endOfMonth(-2) AND  labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved >= startOfMonth(-2) AND resolved <= endOfMonth(-2) AND  labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved >= startOfMonth(-2) AND resolved <= endOfMonth(-2) AND  labels = MSS".to_owned(),
    ])
}

// Closing balances

pub fn closing_balance_current_month() -> Report {
    Report(vec![
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = High AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Medium AND resolved = NULL AND labels = MSS".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved = NULL AND labels = JPC".to_owned(),
        "jql=project = WEBOPS AND issuetype = Story AND priority = Low AND resolved = NULL AND labels = MSS".to_owned(),
    ])
}

pub fn closing_balance_prior_month_one() -> Report {
    Report(vec![
        "jql=project=webops and issuetype=story and created <= endofmonth(-1) and priority=high and (resolved = null or resolved >= endofmonth(-1)) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-1) and priority=high and (resolved = null or resolved >= endofmonth(-1)) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-1) and priority=medium and (resolved = null or resolved >= endofmonth(-1)) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-1) and priority=medium and (resolved = null or resolved >= endofmonth(-1)) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-1) and priority=low and (resolved = null or resolved >= endofmonth(-1)) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-1) and priority=low and (resolved = null or resolved >= endofmonth(-1)) and labels = mss".to_owned(),
    ])
}

pub fn closing_balance_prior_month_two() -> Report {
    Report(vec![
        "jql=project=webops and issuetype=story and created <= endofmonth(-2) and priority=high and (resolved = null or resolved >= endofmonth(-2)) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-2) and priority=high and (resolved = null or resolved >= endofmonth(-2)) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-2) and priority=medium and (resolved = null or resolved >= endofmonth(-2)) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-2) and priority=medium and (resolved = null or resolved >= endofmonth(-2)) and labels = mss".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-2) and priority=low and (resolved = null or resolved >= endofmonth(-2)) and labels = jpc".to_owned(),
        "jql=project=webops and issuetype=story and created <= endofmonth(-2) and priority=low and (resolved = null or resolved >= endofmonth(-2)) and labels = mss".to_owned(),
    ])
}
