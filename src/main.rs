fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(link) = args.get(1) {
        println!("Link given: {}", link);
    } else {
        println!("No link given.");
    }
}
