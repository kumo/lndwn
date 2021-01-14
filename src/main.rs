use regex::Regex;
use std::io::Cursor;
use std::io::Read;

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

/*fn get_sticker_pack_link(id: &str) -> Option<&str> {
    let link = format!("http://dl.stickershop.line.naver.jp/products/0/0/1/{}/iphone/stickers@2x.zip", id).as_str();

    return Some(link);
}*/

struct StickerPack {
	name: String,
}

fn get_sticker_pack(link: &str) -> Option<StickerPack> {
    let id = get_sticker_pack_id(link).expect("Couldn't find ID in link");

    let link = format!("http://dl.stickershop.line.naver.jp/products/0/0/1/{}/iphone/stickers@2x.zip", id);

    println!("Download link: {}", link);

    let mut resp = reqwest::blocking::get(&link).unwrap();

    if resp.status().is_success() {
        println!("The request was successful!");
        println!("Response: {:?}", resp);

        let mut buf: Vec<u8> = vec!();
        resp.copy_to(&mut buf).unwrap();

        let reader = Cursor::new(buf);

        let mut archive = zip::ZipArchive::new(reader).unwrap();

        let mut file = archive.by_name("productInfo.meta").unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();

        println!("{:?}", file_contents);

        let meta = json::parse(&file_contents).unwrap();

        println!("Title is {}", meta["title"]["en"]);
        println!("Rest is {}", meta);

        Some(StickerPack { name: meta["title"]["en"].to_string() })
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let link = args.get(1).expect("You didn't specify a link");

    let sticker_pack = get_sticker_pack(link).expect("Sticker pack not found");

    println!("Sticker pack is {}", sticker_pack.name);
}
