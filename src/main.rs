use rand::Rng;

fn get_version() -> f32 {
    return rand::thread_rng().gen_range(0.0, 20.0);
}

fn usage() {
    let version: f32 = get_version();

    println!("Markdown compiler written by Paul Chong");
    println!("Version {}", version);
}

fn main() {
    usage();
}
