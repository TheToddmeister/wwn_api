use std;

use envy;
use reqwest::{self, Client, header::HeaderMap};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use url;

pub struct NveStruct {
    pub client: Client,
    pub url:  url::Url,
    pub header: HeaderMap
}

#[derive(Deserialize, Serialize)]
struct APIConfig {
    nve_key: String,
    test: String,
}

fn read_from_env() -> APIConfig {
    let var = envy::from_env::<APIConfig>().unwrap();
    return var;
}

fn nve_key() -> String {
    //Placeolder
    let key = "zYBdyjn870SQaYL4vL3nkQ==".to_string();
    return key;
}
fn build_client() -> Client {
    let default_headers = buid_nve_headers();
    let timeout = std::time::Duration::new(5,0);
    let client = reqwest::Client::builder()
        .default_headers(default_headers)
        .build()
        .unwrap();
    return client;
}

fn build_base_nve_url() -> url::Url {
    let base_nve_url =  url::Url::parse("https://hydapi.nve.no/api/v1/").expect("Failed to parse Url");
    return base_nve_url;
}


fn buid_nve_headers() -> HeaderMap {
    let key = HeaderValue::from_maybe_shared(nve_key()).unwrap();
    let response_format = HeaderValue::from_static("application/json");

    let mut headers = HeaderMap::new();
    headers.insert("X-API-Key", key);
    headers.insert("accept", response_format);
    return headers;
}

pub fn build_nve_httpclient()-> NveStruct {
    let client = build_client();
    let url = build_base_nve_url();
    let body = buid_nve_headers();

    let nve = NveStruct {
        client,
        url,
        header: body
    };
    return nve;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_environment(){
        let var = read_from_env().test;
        assert_eq!("Test", var)
    }
}