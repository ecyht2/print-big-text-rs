//! A module containing functions that generates some useful [CharacterMap]s used by
//! the [BigText](crate::BigText) struct.

use serde_json::Result;
use std::collections::HashMap;

static LETTERS: &str = include_str!("letters.json");
static DIGITS: &str = include_str!("digits.json");
static PUNCTUATION: &str = include_str!("punctuation.json");
static WHITESPACE: &str = include_str!("whitespace.json");

/// The [BigText](crate::BigText) type used by BigText struct.
pub type CharacterMap = HashMap<char, [String; 5]>;

/// Returns a [CharacterMap] only containing asii letters.
pub fn ascii_letters() -> CharacterMap {
    serde_json::from_str(LETTERS).unwrap()
}

/// Returns a [CharacterMap] only containing digits.
pub fn digits() -> CharacterMap {
    from_json(DIGITS).unwrap()
}

/// Returns a [CharacterMap] only containing punctuations.
pub fn punctuation() -> CharacterMap {
    from_json(PUNCTUATION).unwrap()
}

/// Returns a [CharacterMap] only containing whitepaces.
pub fn whitespace() -> CharacterMap {
    from_json(WHITESPACE).unwrap()
}

/// Returns a [CharacterMap] containting all the characters of the previous maps.
pub fn printables() -> CharacterMap {
    let mut printables: CharacterMap = HashMap::new();

    printables.extend(ascii_letters());
    printables.extend(digits());
    printables.extend(punctuation());
    printables.extend(whitespace());

    printables
}

/// Creates a [CharacterMap] from a JSON string.
fn from_json(map_data: &str) -> Result<CharacterMap> {
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
