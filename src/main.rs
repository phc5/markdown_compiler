use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v.");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str(") ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by {}\nHomepage: {}\nUsage: mrkdwn <filename>.md",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("Parsing {}...", filename);

    let input_filename = Path::new(filename);

    let file = match File::open(&input_filename) {
        Ok(val) => val,
        Err(err) => panic!("[ERROR] Could not open file: {}", err),
    };

    // Create a Vector of Strings to store tokens
    let mut tokens: Vec<String> = Vec::new();

    // Read the file line-by-line
    let reader = BufReader::new(file);

    // Keep track of opened/closed tags
    let mut _htag: bool = false;
    let mut _ptag: bool = false;

    for line in reader.lines() {
        let line_content = line.unwrap();

        let mut first_char: Vec<char> = line_content.chars().take(1).collect();
        let mut output_line = String::new();
        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_content[2..]);
            }
            _ => {
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line_content);
            }
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename.to_string()).expect("[Error] Could not create ouput file.");

    for token in &tokens {
        outfile
            .write_all(token.as_bytes())
            .expect("[Error] Could not write to output file.");
    }
    println!("Parsing complete!");
}

// When user enters a markdown file
//      Open file
//      Parse file line by line into a buffer
//      Export buffer to a new html file
// Other args
//      Show the banner
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => print_long_banner(),
    };
}
