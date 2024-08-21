use regex::Regex;
use std::str::FromStr;

fn parse_kv<T>(line: &str, key: &str, separator: Option<&str>) -> Option<T>
where
    T: FromStr,
{
    let regex =
        Regex::new(format!(r"{}.*\s*{}\s*(.+)", key, separator.unwrap_or(" ")).as_str()).unwrap();
    if let Some(caps) = regex.captures(line) {
        if let Ok(value) = caps[1].parse::<T>() {
            return Some(value);
        }
    }

    None
}

pub fn parse_kv_separated_space<T>(line: &str, key: &str) -> Option<T>
where
    T: FromStr,
{
    if let Some(value) = parse_kv::<T>(line, key, None) {
        return Some(value);
    }

    None
}
