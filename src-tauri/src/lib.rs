extern crate reqwest;

pub mod http_util {

    use mime::APPLICATION_JSON;
    use reqwest::header::{CONTENT_TYPE, USER_AGENT};

    const UA: &'static str =
        "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0";

    pub fn req_get(request_url: &str) -> Result<String, String> {
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

    pub fn req_post_json(request_url: &str, params: &str) -> Result<String, String> {
        let client = reqwest::blocking::Client::new();

        let res = client
            .post(request_url)
            .header(USER_AGENT, UA)
            .header(CONTENT_TYPE, APPLICATION_JSON.to_string())
            .json(params)
            .send()
            .unwrap()
            .text()
            .unwrap();

        Ok(res)
    }
}
