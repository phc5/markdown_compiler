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

fn main() {
    print_long_banner();
}

// What's happenting here is that the string "0.1" is compiled into
// the program as a string literal in stack memory. While version
// is dynamically allocated in heap memory at runtime. Version borrows
// the value at the address in stack memory.
// let version = "0.1"
