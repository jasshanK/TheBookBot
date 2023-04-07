use crate::bot::verse::types::Page;

use reqwest::StatusCode;
use scraper::{Html, Selector};
use regex::Regex;
use rand::prelude::*;

mod utils;
pub mod types;

fn build_page(title: &str, url: &str) -> Page {
    Page{
        title: title.to_string(),
        url: url.to_string()
    }
}

pub async fn fetch_pages(seed: &str, pages: &mut Vec<Page>){
    let client = utils::get_client();
    let result = client.get(seed).send().await.unwrap();
    let raw_html = match result.status() {
            StatusCode::OK => result.text().await.unwrap(),
            _ => panic!("Something went wrong"),
        };

    let document = Html::parse_document(&raw_html);

    let a_selector = Selector::parse("a").unwrap();

    let get_title_re = Regex::new(r"g>.*").unwrap();
    let replace_re = Regex::new(r"[</>]").unwrap();

    for element in document.select(&a_selector) { 
        let line = element.inner_html();

        let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no URL found",
        };

        match get_title_re.captures_iter(&line).next(){
            Some(cap) => {
                let n = cap.len() - 1;
                let replaced = replace_re.replace_all(&cap[n], "");

                let page: Page = build_page(&replaced[2..], &href);
                pages.push(page);
            }
            _ => ()
        }
    }
}

pub fn generate_page(pages: &mut Vec<Page>) -> Page {
    let n = pages.len();
    let mut rng = thread_rng();
    let x = rng.gen_range(0..n-1);
    
    build_page(&pages[x].title, &pages[x].url)
}