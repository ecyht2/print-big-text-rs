//! A module containing functions that generates some useful character_maps used by
//! the BigText struct.

use serde_json::Value;
use std::collections::HashMap;

static LETTERS: &str = include_str!("letters.json");
static DIGITS: &str = include_str!("digits.json");
static PUNCTUATION: &str = include_str!("punctuation.json");
static WHITESPACE: &str = include_str!("whitespace.json");

/// Returns a `character_map` only containing asii letters.
pub fn ascii_letters() -> HashMap<char, [String; 5]> {
    from_json(LETTERS)
}

/// Returns a `character_map` only containing digits.
pub fn digits() -> HashMap<char, [String; 5]> {
    from_json(DIGITS)
}

/// Returns a `character_map` only containing punctuations.
pub fn punctuation() -> HashMap<char, [String; 5]> {
    from_json(PUNCTUATION)
}

/// Returns a `character_map` only containing whitepaces.
pub fn whitespace() -> HashMap<char, [String; 5]> {
    from_json(WHITESPACE)
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
fn from_json(map_data: &str) -> HashMap<char, [String; 5]> {
    let map_data: Value = serde_json::from_str(map_data).expect("Invalid JSON String");
    let v: &serde_json::Map<String, Value> = map_data.as_object().unwrap();

    let mut map: HashMap<char, [String; 5]> = HashMap::new();

    for (character, array) in v {
        let character = character.chars().nth(0).expect("Invalid Character Index");
        let array = array.as_array().expect("Invalid Array");

        // Converting Array to appropriate array
        let mut output_array: [String; 5] = [
            String::from("     "),
            String::from("     "),
            String::from("     "),
            String::from("     "),
            String::from("     "),
        ];

        for (i, str) in array.iter().enumerate() {
            output_array[i] = str.as_str().expect("Invalid Array String").to_string();
        }

        // Inserting value
        map.insert(character, output_array);
    }

    map
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

        let json_map = from_json(json_data);
        assert_eq!(json_map, map);
    }
}
