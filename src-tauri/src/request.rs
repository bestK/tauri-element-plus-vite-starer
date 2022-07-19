//! 网络请求模块
extern crate reqwest;

use reqwest::header::{CONTENT_TYPE, USER_AGENT};

const UA: &'static str =
    "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0";

/// # get request
///
/// ### Example
/// ```rust
/// get("https://api.github.com/users/bestK")
/// ```
///
pub fn get(request_url: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get(request_url)
        .header(USER_AGENT, UA)
        .send()
        .unwrap()
        .text()
        .unwrap();

    Ok(res)
}

/// # post request for json
///
/// ### Example
/// ```rust
/// post_json("https://api.github.com/orgs/ORG/repos","{\"foo(福)\":\"bar(报)\"}")
/// ```
///
pub fn post_json(request_url: &str, params: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(request_url)
        .header(USER_AGENT, UA)
        .header(CONTENT_TYPE, "application/json")
        .json(params)
        .send()
        .unwrap()
        .text()
        .unwrap();

    Ok(res)
}
