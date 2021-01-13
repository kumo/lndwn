use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_from_url_extraction_short() {
        let url = "https://store.line.me/stickershop/product/12969848/ja";
        let id = get_sticker_pack_id(url);

        assert_eq!(id, "12969848");
    }
}

fn get_sticker_pack_id(url: &str) -> &str {
    let re = Regex::new(r"line[^0-9]*([0-9]+)").expect("Regular expression is invalid");

    if let Some(captures) = re.captures(url) {
        if let Some(id) = captures.get(1) {
            println!("ID found");
            return id.as_str();
        }
    }

    println!("No ID found");
    return ""
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(link) = args.get(1) {
        println!("Link given: {}", link);
    } else {
        println!("No link given.");
    }
}
