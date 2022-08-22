use regex::Regex;
use std::io::Cursor;
use std::io::Read;
use serde::{Deserialize, Serialize};

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

    #[test]
    fn test_json_reading() {
        let json_text = r#"
        {
          "packageId": 12969848,
          "title": {
            "en": "Yokai Housemate",
            "ja": "妖怪シェアハウス"
          },
          "stickers": [
            {
              "id": 343491126,
              "width": 179,
              "height": 160
            },
            {
              "id": 343491127,
              "width": 181,
              "height": 160
            }
          ]
        }
        "#;
        
        let sticker_pack: StickerPack = serde_json::from_str(&json_text).unwrap();

        assert_eq!(sticker_pack.title.en, Some(String::from("Yokai Housemate")));
        assert_eq!(sticker_pack.title.ko, None);
        assert_eq!(sticker_pack.stickers.len(), 2);
    }
}

fn get_sticker_pack_id(url: &str) -> Option<&str> {
    Some(
        Regex::new(r"line[^0-9]*([0-9]+)")
            .expect("Regular expression is invalid")
            .captures(url)?
            .get(1)?
            .as_str(),
    )
}

/*fn get_sticker_pack_link(id: &str) -> Option<&str> {
    let link = format!("http://dl.stickershop.line.naver.jp/products/0/0/1/{}/iphone/stickers@2x.zip", id).as_str();

    return Some(link);
}*/

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StickerPack {
    package_id: usize,
    title: StickerPackTitle,
    stickers: Vec<StickerPackSticker>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StickerPackTitle {
    en: Option<String>,
    ja: Option<String>,
    ko: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StickerPackSticker {
    id: usize,
    width: usize,
    height: usize,
}

fn get_sticker_pack(link: &str) -> Option<StickerPack> {
    let id = get_sticker_pack_id(link).expect("Couldn't find ID in link");

    let link = format!(
        "http://dl.stickershop.line.naver.jp/products/0/0/1/{}/iphone/stickers@2x.zip",
        id
    );

    println!("Download link: {}", link);

    let mut resp = reqwest::blocking::get(&link).unwrap();

    if resp.status().is_success() {
        println!("The request was successful!");
        println!("Response: {:?}", resp);

        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf).unwrap();

        let reader = Cursor::new(buf);

        let mut archive = zip::ZipArchive::new(reader).unwrap();

        let mut file = archive.by_name("productInfo.meta").unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();

        println!("{:?}", file_contents);

        let sticker_pack: StickerPack = serde_json::from_str(&file_contents).unwrap();

        println!("The data is {:#?}", sticker_pack);

        for sticker in &sticker_pack.stickers {
            println!("Sticker is {:?}", sticker);
        }

        Some(sticker_pack)
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let link = args.get(1).expect("You didn't specify a link");

    let sticker_pack = get_sticker_pack(link).expect("Sticker pack not found");

    println!("Sticker pack is {:?}", sticker_pack.title.en);
}
