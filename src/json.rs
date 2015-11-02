/// Count of stories returned by jql query
///
/// Honestly, none of these stupid fields are used except for total, but `serde` absolutely refuses
/// to deal with any field it can't identify, unlike every other deserialization library on the
/// face of the planet...
#[derive(Deserialize)]
pub struct StorySummary {
    #[serde(rename="startAt")]
    start_at: i32,

    #[serde(rename="maxResults")]
    max_results: i32,

    total: i32,
    issues: Vec<()>,
}

impl StorySummary {
    pub fn total(&self) -> i32 { self.total }
}
