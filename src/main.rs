use crate::logs::{format_error, print_info, CONFIG_ERROR, EXPORT_ERROR, SCRAPER_ERROR, SUCCESS_MESSAGE};
use crate::models::{Page, SheetList};

mod models;
mod scraper;
mod export;
mod config;
mod logs;

#[tokio::main]
async fn main() {
    let page: Page = match config::read_config() {
        Ok(page) => page,
        Err(_) => panic!("{}", format_error(CONFIG_ERROR))
    };
    let sheets: SheetList = match scraper::use_web_scraper(page).await {
        Ok(sheets) => sheets,
        Err(_) => panic!("{}", format_error(SCRAPER_ERROR))
    };
    match export::export_csv_file(sheets) {
        Ok(_) => print_info(SUCCESS_MESSAGE),
        Err(_) => panic!("{}", format_error(EXPORT_ERROR))
    };
}
