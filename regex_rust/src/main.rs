use fancy_regex::Regex;
use json::load_json_input;

pub(crate) mod json;

fn main() {
    let regex_pattern = r#"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\r\n\p{L}\p{N}]?\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+"#;
    let regex = Regex::new(regex_pattern).expect("Error parsing the regex");
    println!("{:?}", load_json_input());
}
