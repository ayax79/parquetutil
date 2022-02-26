use parquet::errors::ParquetError;
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::path::Path;
use std::io::{Result as IOResult, Error as IOError};

pub fn display_info(file_path: &str) -> IOResult<()> {

    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let reader = SerializedFileReader::new(file).map_err(to_io_error)?;
    let parquet_metadata = reader.metadata();   
    let row_group_reader = reader.get_row_group(0).unwrap();

    let num_row_groups = parquet_metadata.num_row_groups();    
    
    println!("num row groups: {num_row_groups}");

    let num_columns = row_group_reader.num_columns();
    println!("num columns {num_columns}");

    Ok(())
}

pub fn cat(file_path: &str) -> IOResult<()> {

    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = SerializedFileReader::new(file).map_err(to_io_error)?;
    let iterator = reader.get_row_iter(None).map_err(to_io_error)?;
    for row in iterator {
        for (idx, (name, field)) in row.get_column_iter().enumerate() {
            println!("column index: {}, column name: {}, column value: {}", idx, name, field);
        }
    }
    Ok(())
}


fn to_io_error(error: ParquetError) -> IOError {
    IOError::new(std::io::ErrorKind::Other, error)
}