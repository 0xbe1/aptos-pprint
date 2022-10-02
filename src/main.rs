use regex::{Captures, Regex};
use std::io::{self, BufRead};

const DEBUG_PREFIX: &str = "[debug] (&)";

#[derive(Debug, Clone, PartialEq)]
struct ParseError {
    msg: String,
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from stdin");
        if line.starts_with(DEBUG_PREFIX) {
            println!("{}", pprint(line))
        } else {
            println!("{}", line);
        }
    }
}

fn pprint(line: String) -> String {
    let re = Regex::new(r"\[(\d+,\s)+\d+]").unwrap();
    let result = re.replace(line.as_str(), |caps: &Captures| {
        let s = caps.get(0).map_or("", |m| m.as_str());
        parse_utf8_arr(s).unwrap_or(s.to_string())
    });
    format!("{}", result)
}

fn parse_utf8_arr(s: &str) -> Result<String, ParseError> {
    let s_arr: Vec<&str> = s[1..s.len() - 1].split(", ").collect();
    let u8_arr = s_arr
        .iter()
        .filter_map(|ss| ss.parse().ok())
        .collect::<Vec<u8>>();
    if s_arr.len() > u8_arr.len() {
        Err(ParseError {
            msg: "not an u8 array".to_string(),
        })
    } else {
        String::from_utf8(u8_arr).or(Err(ParseError {
            msg: "u8 array => str nok".to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_utf8_arr, pprint, ParseError};

    #[test]
    fn pprint_ok() {
        assert_eq!(
            pprint("[debug] (&) [97, 112, 116, 111, 115, 32, 100, 101, 98, 117, 103, 32, 109, 97, 100, 101, 32, 101, 97, 115, 121]".to_string()),
            "[debug] (&) aptos debug made easy"
        );
        assert_eq!(
            pprint("[debug] (&) { [97, 112, 116, 111, 115, 32, 100, 101, 98, 117, 103, 32, 109, 97, 100, 101, 32, 101, 97, 115, 121] }".to_string()),
            "[debug] (&) { aptos debug made easy }"
        );
        assert_eq!(
            pprint("[debug] (&) [0, 300]".to_string()),
            "[debug] (&) [0, 300]"
        );
    }

    #[test]
    fn parse_ok() {
        assert_eq!(
            parse_utf8_arr("[97, 112, 116, 111, 115, 32, 100, 101, 98, 117, 103, 32, 109, 97, 100, 101, 32, 101, 97, 115, 121]"),
            Ok("aptos debug made easy".to_string())
        );
        assert_eq!(
            parse_utf8_arr("[0, 300]"),
            Err(ParseError {
                msg: "not an u8 array".to_string()
            })
        );
    }
}
