use crate::models::{Page, SheetList};

mod models;
mod scraper;
mod export;
mod config;

#[tokio::main]
async fn main() {
    let page: Page = match config::read_config() {
        Ok(page) => page,
        Err(_) => panic!("Error while retrieving page content.")
    };
    let sheets: SheetList = match scraper::use_web_scraper(page).await {
        Ok(sheets) => sheets,
        Err(_) => panic!("Error while scraping page.")
    };
    match export::export_csv_file(sheets) {
        Ok(_) => println!("Done."),
        Err(_) => panic!("Error while exporting scraped data in csv file.")
    };
}
