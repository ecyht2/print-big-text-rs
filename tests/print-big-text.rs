use std::collections::HashMap;

use print_big_text_rs::BigText;

#[test]
fn test_text() {
    let printer = BigText::new("Ab1", None);
    assert_eq!(printer.text(), "Ab1");
}

#[test]
fn test_supported_characters() {
    let _printer = BigText::new("Ab1", None);
}

#[test]
fn test_set_text() {
    let mut printer = BigText::new("Ab1", None);
    printer.set_text("cY@");
    assert_eq!(printer.text(), "cY@");
}

#[test]
fn test_print() -> Result<(), std::io::Error> {
    let mut vec = Vec::new();
    let printer = BigText::new("A", None);
    printer.print(Some(&mut vec))?;
    // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
    let str = String::from_utf8(vec).unwrap_or_default();
    println!("{}", str);

    assert_eq!(" ***  \n*   * \n***** \n*   * \n*   * \n", str);
    Ok(())
}

#[test]
fn test_character_map() {
    let map: HashMap<char, [String; 5]> = HashMap::from([
        (
            'A',
            [
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
            ],
        ),
        (
            '1',
            [
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
            ],
        ),
        (
            '"',
            [
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
            ],
        ),
    ]);
    let printer = BigText::new("", Some(map.clone()));
    assert_eq!(&map, printer.character_map())
}

#[test]
fn test_set_character_map() {
    let map: HashMap<char, [String; 5]> = HashMap::from([
        (
            'A',
            [
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
            ],
        ),
        (
            '1',
            [
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
            ],
        ),
        (
            '"',
            [
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
                String::from("     "),
            ],
        ),
    ]);
    let mut printer = BigText::new("", None);
    printer.set_character_map(map.clone());
    assert_eq!(&map, printer.character_map())
}
