use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::Command;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

struct Term {
    width: usize,
    height: usize,
}

fn get_term_dimensions() -> Term {
    //
    let mut term = Term {
        width: 0,
        height: 0,
    };

    for dim in ["cols", "lines"].iter() {
        //
        let cmd = Command::new("tput")
            .arg(dim)
            .output()
            .expect("Failed to read terminal information.");

        let dim_str = String::from_utf8(cmd.stdout).unwrap();
        let dim_int = dim_str.trim().parse::<usize>().unwrap();

        if dim == &"lines" {
            term.height = dim_int - 5;
        } else {
            term.width = dim_int;
        }
    }

    term
}

fn get_chars(line: &str) -> Vec<&str> {
    //
    let gr = UnicodeSegmentation::graphemes(line, true).collect::<Vec<&str>>();

    gr
}

fn get_seperator(chunk_count: &usize) -> String {
    //
    let a = "\x1B[1;100m";
    let b = "chunk:";
    let c = "\x1B[0m";
    let sep = format!("{}{}{}{}", a, b, chunk_count, c);

    sep
}

fn get_confirmation(chunk_count: &usize, line_count: &usize, col_count: &usize) {
    //
    let sep = get_seperator(&chunk_count);
    // let sep_debug = format!("{}-{}-{}", sep, line_count, col_count);

    // We need to write the "chunk" title into /dev/tty. This is the only way to
    // print it to the terminal without additional line breaks.
    fs::write("/dev/tty", sep).expect("Cannot write to /dev/tty on your system.");

    // Read user input from /dev/tty. If user hits, enter, the main loop
    // contiues. We read from /dev/tty, because stdin is already occupied from
    // from the main loop.
    let f = File::open("/dev/tty").expect("Cannot read /dev/tty on your system.");

    for line_b in BufReader::new(f).lines() {
        //
        if line_b.unwrap() == "" {
            break;
        }
    }
}

fn main() {
    //
    let term = get_term_dimensions();
    let stdin = io::stdin();
    let mut line_count = 0;
    let mut chunk_count = 0;

    for line in stdin.lock().lines() {
        //
        line_count = line_count + 1;

        // Read Unicode graphenes char by char.
        for (col_count, c) in get_chars(&line.unwrap()).iter().enumerate() {
            //
            // If a line exceeds the terminal width, we increase the line count.
            if col_count != 0 && col_count % term.width == 0 {
                line_count = line_count + 1;
            }

            // If the number of lines exceeds the hight of the terminal, we
            // start with a new "chunk".
            if line_count >= term.height {
                if col_count != 0 && col_count % term.width == 0 {
                    print!("\n");
                }

                // Wait for Enter and print "chunk" indicator.
                get_confirmation(&chunk_count, &line_count, &col_count);

                chunk_count = chunk_count + 1;
                line_count = 0;
            }

            print!("{}", c);
        }

        print!("\n");
    }
}
