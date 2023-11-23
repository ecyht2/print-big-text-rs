//! A module containing functions that generates some useful character_maps used by
//! the BigText struct.

use serde_json::Result;
use std::collections::HashMap;

static LETTERS: &str = include_str!("letters.json");
static DIGITS: &str = include_str!("digits.json");
static PUNCTUATION: &str = include_str!("punctuation.json");
static WHITESPACE: &str = include_str!("whitespace.json");

/// Returns a `character_map` only containing asii letters.
pub fn ascii_letters() -> HashMap<char, [String; 5]> {
    serde_json::from_str(LETTERS).unwrap()
}

/// Returns a `character_map` only containing digits.
pub fn digits() -> HashMap<char, [String; 5]> {
    from_json(DIGITS).unwrap()
}

/// Returns a `character_map` only containing punctuations.
pub fn punctuation() -> HashMap<char, [String; 5]> {
    from_json(PUNCTUATION).unwrap()
}

/// Returns a `character_map` only containing whitepaces.
pub fn whitespace() -> HashMap<char, [String; 5]> {
    from_json(WHITESPACE).unwrap()
}

/// Returns a `character_map` containting all the characters of the previous maps.
pub fn printables() -> HashMap<char, [String; 5]> {
    let mut printables: HashMap<char, [String; 5]> = HashMap::new();

    printables.extend(ascii_letters());
    printables.extend(digits());
    printables.extend(punctuation());
    printables.extend(whitespace());

    printables
}

/// Creates a `character_map` from a JSON string.
fn from_json(map_data: &str) -> Result<HashMap<char, [String; 5]>> {
    serde_json::from_str(map_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let map = HashMap::from([
            (
                'A',
                [
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ],
            ),
            (
                '1',
                [
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ],
            ),
            (
                '"',
                [
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ],
            ),
        ]);

        let json_data = "
        {
            \"A\": [\"\", \"\", \"\", \"\", \"\"],
            \"1\": [\"\", \"\", \"\", \"\", \"\"],
            \"\\\"\": [\"\", \"\", \"\", \"\", \"\"]
        }";

        let json_map = from_json(json_data).unwrap();
        assert_eq!(json_map, map);
    }
}
