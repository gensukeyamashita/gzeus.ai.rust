mod common_headers;

pub use common_headers::common_headers;

use crate::settings::Settings;
use reqwest::header::{HeaderMap, HeaderValue};

pub fn common_headers(settings: &Settings) -> HeaderMap {
    let config: HeadersConfig = serde_yaml::from_str(
        &fs::read_to_string("headers_config.yaml").expect("Failed to read headers config file")
    ).expect("Failed to parse headers config file");

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", &settings.api.token)).unwrap());
    headers.insert("Accept", HeaderValue::from_static(&config.accept));
    headers.insert("Accept-Language", HeaderValue::from_static(&config.accept_language));
    headers.insert("Connection", HeaderValue::from_static(&config.connection));
    headers.insert("Content-Length", HeaderValue::from_static(&config.content_length));
    headers.insert("Origin", HeaderValue::from_str(&settings.api.base_url).unwrap());
    headers.insert("Referer", HeaderValue::from_str(&settings.api.base_url).unwrap());
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static(&config.sec_fetch_dest));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static(&config.sec_fetch_mode));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static(&config.sec_fetch_site));
    headers.insert("User-Agent", HeaderValue::from_static(&config.user_agent));
    headers.insert("sec-ch-ua", HeaderValue::from_static(&config.sec_ch_ua));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static(&config.sec_ch_ua_mobile));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static(&config.sec_ch_ua_platform));
    headers
}