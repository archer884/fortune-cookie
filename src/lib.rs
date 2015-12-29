extern crate hyper;
extern crate regex;

use std::io::Read;

use hyper::Client;
use regex::Regex;

const COOKIE_PATTERN: &'static str = r#"<a.*?class="cookie-link">(.*?)</a>"#;

pub fn cookie() -> Result<String, String> {
    match Client::new().get("http://www.fortunecookiemessage.com/").send() {
        Err(e) => Err(format!("{:?}", e)),
        Ok(mut response) => {
            let mut buf = String::new();
            response.read_to_string(&mut buf).ok();
            extract_fortune(&buf).ok_or("Fortune not found.".to_owned())
        }
    }
}

fn extract_fortune(content: &str) -> Option<String> {
    pattern().captures(content).and_then(|captures|
        captures.at(1).map(|s| s.to_owned())
    )
}

fn pattern() -> Regex {
    Regex::new(COOKIE_PATTERN).unwrap()
}

#[cfg(test)]
mod tests {
    use super::COOKIE_PATTERN;
    use regex::Regex;

    #[test]
    fn cookie_pattern_is_valid() {
        Regex::new(COOKIE_PATTERN).unwrap();
    }
}
