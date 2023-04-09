mod data_source;
mod data_source_decorator;
mod file_data_source;
mod encryption_data_source_decorator;
mod compression_data_source_decorator;

use std::io::{BufReader, Cursor, Read};
use crate::pattern::decorator::compression_data_source_decorator::CompressionDataSource;
use crate::pattern::decorator::data_source::DataSource;
use crate::pattern::decorator::data_source_decorator::DataSourceDecorator;
use crate::pattern::decorator::encryption_data_source_decorator::EncryptionDataSourceDecorator;
use crate::pattern::decorator::file_data_source::FileDataSource;


#[test]
fn test_decorator() {
    let file_data_source = FileDataSource::default();
    let compression_decorator = CompressionDataSource::wrap(file_data_source);
    let encryption_decorator = EncryptionDataSourceDecorator::wrap(compression_decorator);
    let data_source = encryption_decorator;
    data_source.read();
    println!("=================");
    data_source.write();
}








#[test]
fn test_buf_reader() {
    let mut input = BufReader::new(Cursor::new("Input data"));
    let mut buf = [0; 10];

    input.read(&mut buf).ok();
    for byte in buf {
        println!("{}", char::from(byte))
    }

}