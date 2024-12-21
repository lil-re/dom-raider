use std::fs::File;
use csv::{Error, Writer};
use crate::logs::{format_error, CELLS_EXPORT_ERROR, HEADERS_EXPORT_ERROR};
use crate::models::{FileItem, FileItemType, SheetList};

/// Export scraped content in CSV files
///
/// It iterates through each sheet data and starts by exporting the headers and continues with the
/// file items
pub(crate) fn export_csv_file (sheets: SheetList) -> Result<(), Error> {
  for (title, file_items) in sheets {
    let file_name = format!("{}.csv", title);
    let mut wtr = csv::Writer::from_path(file_name)?;
    export_headers(&file_items, &mut wtr);
    export_file_items(file_items, &mut wtr);
    wtr.flush()?;
  }
  Ok(())
}

/// Retrieve the first line of a sheet and use its cells to build the headers
fn export_headers<'a>(file_items: &Vec<FileItem>, wtr: &mut Writer<File>) {
  let first_line = file_items.get(0);
  let mut headers = vec![];

  for cell in &first_line.unwrap().children {
    if cell.item_type == FileItemType::Cell {
      headers.push(&cell.title);
    }
  }
  wtr.write_record(headers).expect(format_error(HEADERS_EXPORT_ERROR).as_str());
}

/// Iterate through a vector of file items to export their contents
fn export_file_items(file_items: Vec<FileItem>, mut wtr: &mut Writer<File>) {
  for child_item in file_items {
    export_file_item(child_item, &mut wtr);
  }
}

/// Retrieve the type of file item to determine how to export its content
fn export_file_item(file_item: FileItem, wtr: &mut Writer<File>) {
  if file_item.item_type == FileItemType::Group {
    export_file_items(file_item.children, wtr)
  } else if file_item.item_type == FileItemType::Line {
    export_cells(file_item, wtr);
  }
}

/// Retrieve the cells and add a new line in the CSV file
fn export_cells(file_item: FileItem, wtr: &mut Writer<File>) {
  let mut cells = vec![];

  for cell in file_item.children {
    if cell.item_type == FileItemType::Cell {
      cells.push(cell.content);
    }
  }
  wtr.write_record(cells).expect(format_error(CELLS_EXPORT_ERROR).as_str());
}
