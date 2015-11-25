use std::fmt;
use super::Report;

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
