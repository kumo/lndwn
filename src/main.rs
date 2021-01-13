use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_from_long_store_url() {
        let url = "https://store.line.me/stickershop/product/12969848/ja";
        let id = get_sticker_pack_id(url);

        assert_eq!(id, Some("12969848"));
    }

    #[test]
    fn test_id_from_short_store_url() {
        let url = "line://shop/detail/12969848";
        let id = get_sticker_pack_id(url);

        assert_eq!(id, Some("12969848"));
    }

    #[test]
    fn test_id_from_share_link_url() {
        let url = "https://line.me/S/sticker/12969848?lang=en&ref=lsh_stickerDetail";
        let id = get_sticker_pack_id(url);

        assert_eq!(id, Some("12969848"));
    }

    #[test]
    fn test_youtube_url() {
        let url = "https://www.youtube.com/watch?v=e_V9RBcgwHI";
        let id = get_sticker_pack_id(url);

        assert_eq!(id, None);
    }

    #[test]
    fn test_line_url() {
        let url = "http://line-stickers.com";
        let id = get_sticker_pack_id(url);

        assert_eq!(id, None);
    }
}

fn get_sticker_pack_id(url: &str) -> Option<&str> {
    Some(Regex::new(r"line[^0-9]*([0-9]+)")
        .expect("Regular expression is invalid")
        .captures(url)?
        .get(1)?
        .as_str())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(link) = args.get(1) {
        println!("Link given: {}", link);
    } else {
        println!("No link given.");
    }
}
