//! A rust library that prints a given text in it's ascii-art form.
//!

#[allow(unused)]
use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Error, Write},
};

use character_maps::CharacterMap;
pub mod character_maps;

/// A struct that prints strings in it's ascii-art form.
///
/// The struct decides how to print a given character in the ascii-art form via a
/// [CharacterMap]. It is a [HashMap<char, [String, 5]>](std::collections::HashMap)
/// where the keys is the character that is being printed and the values is an
/// [array] of 5 [String] where at each index is what will be printed at each row
/// when printing the ascii-art.
///
/// If the character in the currently stored string to print isn't in the supported
/// characters (not a key in the [CharacterMap]) it will print as a blank character
/// (5 spaces).
///
/// # Examples
/// ```rust
/// use print_big_text_rs::BigText;
/// let printer = BigText::new("HI", None);
/// printer.print(None);
/// ```
///
/// [BigText] struct also implements [Display] trait. This allows printing of the ascii-art
/// using macros provided by [std].
///
/// ```rust
/// use print_big_text_rs::BigText;
/// let printer = BigText::new("HI", None);
/// println!("{}", printer);
/// ```
pub struct BigText {
    /// The current text being stored.
    text: String,
    /// All the characters that can be printed.
    supported_characters: String,
    /// The chracter map used to decide how to print the ASCII text.
    character_map: CharacterMap,
}

impl BigText {
    /// Constructor Function for [BigText].
    ///
    /// If the `character_map` is [None], it defaults to [character_maps::printables()].
    ///
    /// # Examples
    /// ```rust
    /// use print_big_text_rs::BigText;
    ///
    /// let printer: BigText = BigText::new("HI", None);
    /// ```
    ///
    /// [BigText] can also be created with custom maps:
    ///
    /// ```rust
    /// use print_big_text_rs::{character_maps::CharacterMap, BigText};
    /// use std::collections::HashMap;
    ///
    /// let map: CharacterMap = HashMap::from([
    ///     (
    ///         'H',
    ///         [
    ///             String::from("H   H"),
    ///             String::from("H   H"),
    ///             String::from("HHHHH"),
    ///             String::from("H   H"),
    ///             String::from("H   H"),
    ///         ],
    ///     ),
    ///     (
    ///         'i',
    ///         [
    ///             String::from("IIIII"),
    ///             String::from("  I  "),
    ///             String::from("  I  "),
    ///             String::from("  I  "),
    ///             String::from("IIIII"),
    ///         ],
    ///     ),
    /// ]);
    ///
    /// let printer: BigText = BigText::new("HI", Some(map));
    /// ```
    pub fn new(text: &str, character_map: Option<CharacterMap>) -> Self {
        let text = String::from(text);

        // Setting map to default map if None is given
        let character_map = match character_map {
            None => character_maps::printables(),
            Some(map) => map,
        };

        // Getting supported charaters
        let supported_characters = Self::get_supported_characters(&character_map);

        Self {
            text,
            supported_characters,
            character_map,
        }
    }

    /// Gets the text currently text stored in the struct.
    ///
    /// # Examples
    /// ```rust
    /// use print_big_text_rs::BigText;
    ///
    /// let printer = BigText::new("HI", None);
    /// assert_eq!("HI", printer.text());
    /// ```
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    /// Gets the all characters that the struct can printed.
    ///
    /// # Examples
    /// ```rust
    /// use print_big_text_rs::BigText;
    ///
    /// let printer = BigText::new("HI", None);
    ///
    /// println!("{}", printer.supported_characters());
    /// ```
    ///
    /// This should show something simillar but in a different order:
    /// ```text
    /// *C#6]1@%"UAW,^QV(T;NIHREG4)OK3DZL0P?8&BJ72[S.$M!Y5F9X
    /// ```
    pub fn supported_characters(&self) -> &str {
        self.supported_characters.as_ref()
    }

    /// Sets the text currently to print.
    ///
    /// # Examples
    /// ```rust
    /// use print_big_text_rs::BigText;
    /// let mut printer = BigText::new("69", None);
    /// assert_eq!("69", printer.text());
    ///
    /// printer.set_text("420");
    /// assert_eq!("420", printer.text());
    /// ```
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = String::from(text);
        self
    }

    /// Prints the stored string.
    ///
    /// If [None] is provided for stream, the standard output would be used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use print_big_text_rs::BigText;
    /// let printer = BigText::new("A1?", None);
    /// printer.print(None).unwrap();
    /// ````
    ///
    /// This should print the ascii art version to standard output:
    /// ```text
    ///  ***      * ****
    /// *   *     *     *
    /// *****     *   **
    /// *   *     *
    /// *   *     *   *
    /// ```
    ///
    /// [BigText::print()] can be used to print to any structs that implements [std::io::Write]
    /// trait. The example below shows [BigText] printing to a [`Vec<T>`](Vec).
    ///
    /// ```rust
    /// use print_big_text_rs::BigText;
    ///
    /// let mut vec = Vec::new();
    /// let printer = BigText::new("A1?", None);
    /// printer.print(Some(&mut vec)).unwrap();
    /// let str = String::from_utf8(vec).unwrap_or_default();
    /// println!("{}", str);
    ///
    /// assert_eq!(" ***      * ****  \n*   *     *     * \n*****     *   **  \n*   *     *       \n*   *     *   *   \n", str);
    /// ```
    pub fn print(&self, stream: Option<&mut dyn Write>) -> Result<(), Error> {
        let standard = &mut io::stdout();
        let stream = stream.unwrap_or(standard);

        // Looping over 5 lines
        for row in 0..5 {
            // Looping over the all characters
            for col in self.text().chars() {
                // Printing Characters
                match self.character_map.get(&col) {
                    Some(arr) => write!(stream, "{} ", arr[row])?,
                    None => write!(stream, "      ")?,
                };
            }

            // Printing New Line
            write!(stream, "\n")?;
        }

        Ok(())
    }

    /// Gets all the supported characters in the character_map.
    fn get_supported_characters(map: &CharacterMap) -> String {
        let mut supported_characters = String::new();

        for key in map.keys() {
            supported_characters.push(*key);
        }

        supported_characters
    }

    /// Sets the `character_map` to use when printing.
    ///
    /// # Examples
    /// ```rust
    /// use print_big_text_rs::{character_maps::{self, CharacterMap}, BigText};
    /// use std::collections::HashMap;
    ///
    /// let map: CharacterMap = HashMap::from([
    ///     (
    ///         'A',
    ///         [
    ///             String::from("     "),
    ///             String::from("     "),
    ///             String::from("     "),
    ///             String::from("     "),
    ///             String::from("     "),
    ///         ],
    ///     ),
    /// ]);
    /// let mut printer = BigText::new("", None);
    /// assert_eq!(&character_maps::printables(), printer.character_map());
    ///
    /// printer.set_character_map(map.clone());
    /// assert_eq!(&map, printer.character_map());
    /// ```
    pub fn set_character_map(&mut self, character_map: CharacterMap) {
        self.character_map = character_map;
        // Resetting supported_characters
        self.supported_characters = Self::get_supported_characters(&self.character_map);
    }

    /// Gets the `character_map` to use when printing.
    ///
    /// # Examples
    /// ```rust
    /// use print_big_text_rs::{character_maps, BigText};
    /// use std::collections::HashMap;
    ///
    /// let printer = BigText::new("", None);
    /// assert_eq!(&character_maps::printables(), printer.character_map());
    /// ```
    pub fn character_map(&self) -> &CharacterMap {
        &self.character_map
    }
}

impl Display for BigText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Looping over 5 lines
        for row in 0..5 {
            // Looping over the all characters
            for col in self.text().chars() {
                // Printing Characters
                match self.character_map.get(&col) {
                    Some(arr) => write!(f, "{} ", arr[row])?,
                    None => write!(f, "      ")?,
                };
            }

            // Printing New Line
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_supported_characters() {
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

        let supported_characters = BigText::get_supported_characters(&map);
        assert!(supported_characters.contains("A"));
        assert!(supported_characters.contains("\""));
        assert!(supported_characters.contains("1"));
        assert!(!supported_characters.contains("B"));
    }
}
