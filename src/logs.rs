pub const FETCHING_LOG: &str = "FETCHING => ";
pub const ERROR_LOG: &str = "   ERROR => ";
pub const INFO_LOG: &str = "    INFO => ";

pub const CONFIG_ERROR: &str = "Error while retrieving page content.";
pub const SCRAPER_ERROR: &str = "Error while scraping page.";
pub const EXPORT_ERROR: &str = "Error while exporting scraped data in csv file.";
pub const HEADERS_EXPORT_ERROR: &str = "Error while exporting headers";
pub const CELLS_EXPORT_ERROR: &str = "Error while exporting content";

pub const SUCCESS_MESSAGE: &str = "Done.";
pub const STOP_SCRAPER_MESSAGE: &str = "Stopping web scraper.";
pub const CANNOT_FIND_NEXT_PAGE_MESSAGE: &str = "Cannot find next page URL.";
pub const NO_PAGINATION_CONFIG_MESSAGE: &str = "No configuration found for pagination.";

pub fn print_fetching(message: &str) {
  println!("{}{}", FETCHING_LOG, message);
}

pub fn print_info(message: &str) {
  println!("{}{}", INFO_LOG, message);
}

pub fn format_error(message: &str) -> String {
  format!("{}{}", ERROR_LOG, message)
}
