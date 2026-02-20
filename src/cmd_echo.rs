/**
 * @file echo.rs
 *
 * This program is built with IEEE Std 1003.1-2017.
 */
use std::io::Write;

pub fn main(args: &[String]) -> Result<i32, Box<dyn std::error::Error>> {
    let mut iter = args.iter().peekable();

    let mut no_newline = false;

    if let Some(arg) = iter.peek() {
        if arg.as_str() == "-n" {
            no_newline = true;
            iter.next();
        }
    }

    let operands: Vec<&String> = iter.collect();
    print!("{}", operands.iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(" "));

    if no_newline == true {
        std::io::stdout().flush().unwrap();
    } else {
        println!();
    }

    Ok(0)
}