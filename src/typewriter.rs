use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

pub fn typewriter(
    text: &str,
    line_width: usize,
    delays: [u64; 3],
    normalize: bool,
) {
    let mut column = 0;

    // print a single character with the appropriate delay
    let print_character = |character: char, column: &mut usize| {
        print!("{character}");
        io::stdout().flush().unwrap();

        *column += 1;

        let delay = match character {
            '.' | '!' | '?' => delays[2],
            ',' | ';' | ':' => delays[1],
            _ => delays[0],
        };

        thread::sleep(Duration::from_millis(delay));
    };

    if normalize {
        // remove unnecessary whitespaces and newlines
        let normalized_text = text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        for word in normalized_text.split_whitespace() {
            let word_length = word.chars().count();
            let space_needed = if column == 0 { 0 } else { 1 };

            // start a new line if the word does not fit
            if column + space_needed + word_length > line_width && column > 0 {
                println!();
                column = 0;
            }

            if column > 0 {
                print_character(' ', &mut column);
            }

            for character in word.chars() {
                print_character(character, &mut column);
            }
        }
    } else {
        // preserve all original whitespaces and newlines
        for character in text.chars() {
            match character {
                '\n' => {
                    println!();
                    column = 0;
                }
                '\r' => {}
                ' ' | '\t' => {
                    print_character(character, &mut column);
                }
                _ => {
                    // wrap text at the specified line width
                    if column >= line_width && line_width > 0 {
                        println!();
                        column = 0;
                    }

                    print_character(character, &mut column);
                }
            }
        }
    }

    println!("\x1b[0m");
}
