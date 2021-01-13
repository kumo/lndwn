use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_from_url_extraction_short() {
        let url = "https://store.line.me/stickershop/product/12969848/ja";
        let id = get_sticker_pack_id(url);

        assert_eq!(id, Some("12969848"));
    }
}

fn get_sticker_pack_id(url: &str) -> Option<&str> {
    // let id = 
    Some(Regex::new(r"line[^0-9]*([0-9]+)")
        .expect("Regular expression is invalid")
        .captures(url)?
        .get(1)?
        .as_str())
    
    // Some(id)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(link) = args.get(1) {
        println!("Link given: {}", link);
    } else {
        println!("No link given.");
    }
}
