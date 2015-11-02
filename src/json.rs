use hyper::status::StatusCode;
use serde_json;
use serde::de::Deserialize;

pub struct JsonResponse(StatusCode, String);

impl JsonResponse {
    pub fn new(status: StatusCode, body: String) -> JsonResponse {
        JsonResponse(status, body)
    }

    pub fn status(&self) -> StatusCode {
        self.0
    }

    pub fn body(&self) -> &str {
        &self.1
    }

    pub fn to<T: FromJsonResponse>(&self) -> Result<T, T::Error> {
        T::from_response(self)
    }
}

pub trait FromJsonResponse: Sized {
    type Error;

    fn from_response(response: &JsonResponse) -> Result<Self, Self::Error>;
}

impl<T: Deserialize> FromJsonResponse for T {
    type Error = serde_json::error::Error;

    fn from_response(response: &JsonResponse) -> Result<Self, Self::Error> {
        serde_json::from_str(response.body())
    }
}

/// Count of stories returned by jql query
///
/// Honestly, none of these stupid fields are used except for total, but `serde` absolutely refuses
/// to deal with any field it can't identify, unlike every other deserialization library on the
/// face of the planet...
#[allow(unused)]
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
