use crate::models::{FileItem, Page};

mod models;
mod scraper;
mod file;
mod json;

#[tokio::main]
async fn main() {
    let page: Page = match json::read_json() {
        Ok(page) => page,
        Err(_) => panic!("Error while retrieving page content")
    };
    let file_items: Vec<FileItem> = match scraper::use_web_scraper(page).await {
        Ok(file_items) => file_items,
        Err(_) => panic!("Error while scraping page")
    };
    match file::export_csv_file(file_items) {
        Ok(_) => println!("Done"),
        Err(_) => panic!("Error while exporting scraped data in csv file")
    };
}
