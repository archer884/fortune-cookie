extern crate hyper;
extern crate regex;

use std::io::Read;

// Once upon a time, this directive was included to avoid a dependency on OpenSSL in testing.
// Hyper no longer depends on OpenSSL (or at least not directly), and so this is has only
// been left in to make test compilation faster.
#[cfg(not(test))]
use hyper::Client;

use regex::Regex;

const COOKIE_PATTERN: &'static str = r#"<a.*?class="cookie-link">(<p>)?(?P<fortune>.*?)(</p>)?</a>"#;

#[cfg(not(test))]
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
        captures.name("fortune").map(|s| s.as_str().to_string())
    )
}

fn pattern() -> Regex {
    Regex::new(COOKIE_PATTERN).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{COOKIE_PATTERN, extract_fortune};
    use regex::Regex;

    #[test]
    fn cookie_pattern_is_valid() {
        Regex::new(COOKIE_PATTERN).unwrap();
    }

    #[test]
    fn pattern_extracts_valid_fortunes() {
        let content_a = r#"<div id="message"> <div class="quote"><a href="cookie/8386-<p>Your-way-of-doing-what-other-people-do-their-way-is-what-makes-you-special.</p>" class="cookie-link"><p>Your way of doing what other people do their way is what makes you special.</p></a></div>   <div class="bottom-message"> <a href="learn_chinese.php"><strong>Learn Chinese</strong></a>: loan  = hot  asian <br><a href="/lotto_numbers.php"><strong>Lucky numbers (Lotto)</strong></a>: 39-7-17-18-41-24<br><a href="/pick3_numbers.php"><strong>Daily numbers (Pick3)</strong></a>: 840 </div> </div> "#;
        let content_b = r#"<div id="message"> <div class="quote"><a href="cookie/8386-<p>Your-way-of-doing-what-other-people-do-their-way-is-what-makes-you-special.</p>" class="cookie-link">Your way of doing what other people do their way is what makes you special.</a></div>   <div class="bottom-message"> <a href="learn_chinese.php"><strong>Learn Chinese</strong></a>: loan  = hot  asian <br><a href="/lotto_numbers.php"><strong>Lucky numbers (Lotto)</strong></a>: 39-7-17-18-41-24<br><a href="/pick3_numbers.php"><strong>Daily numbers (Pick3)</strong></a>: 840 </div> </div> "#;
        let fortune = "Your way of doing what other people do their way is what makes you special.";

        assert_eq!(fortune, extract_fortune(content_a).unwrap());
        assert_eq!(fortune, extract_fortune(content_b).unwrap());
    }
}
