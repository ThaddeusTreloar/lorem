
pub fn word_title_case(word: &str) -> String {
    let mut chars = word.chars();

    let first = chars.next().unwrap().to_uppercase().to_string();

    first + chars.as_str()
}


pub trait TitleCase {
    fn title_case(&self) -> String;
}

impl TitleCase for String {
    fn title_case(&self) -> String {
        self.split_ascii_whitespace()
        .map(word_title_case)
        .collect::<Vec<String>>()
        .join(" ")
    }
}