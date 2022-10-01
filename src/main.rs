use std::io;
use std::io::BufRead;

const DEBUG_PREFIX: &str = "[debug] (&)";

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from stdin");
        if line.starts_with(DEBUG_PREFIX) {
            println!("[debug] {}", parse_debug_print(line))
        } else {
            println!("{}", line);
        }
    }
}

fn parse_debug_print(line: String) -> String {
    let numbers = &line[DEBUG_PREFIX.len() + 2..line.len() - 1];
    let numbers: Vec<u8> = numbers.split(", ").map(|s| s.parse().unwrap()).collect();
    String::from_utf8(numbers).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parse_debug_print;

    #[test]
    fn parse_ok() {
        assert_eq!(parse_debug_print("[debug] (&) [97, 112, 116, 111, 115, 32, 100, 101, 98, 117, 103, 32, 109, 97, 100, 101, 32, 101, 97, 115, 121]".to_string()), "aptos debug made easy");
    }
}
