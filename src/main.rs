use print_big_text_rs::BigText;
use std::env;

fn main() -> Result<(), std::io::Error> {
    // Intializing the BigText struct
    let mut printer = BigText::new("", None);

    // Getting args and removing the first one (program name)
    let mut args = env::args();
    args.next();

    for i in args {
        // Printing out the string
        println!("string=\"{i}\"");
        // Setting the text and printing the asii-art representation
        printer.set_text(&i).print(None)?;
    }

    Ok(())
}
