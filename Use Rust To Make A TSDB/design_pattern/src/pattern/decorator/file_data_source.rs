use crate::pattern::decorator::data_source::DataSource;

#[derive(Default)]
pub struct FileDataSource {}

impl DataSource for FileDataSource {
    fn write(&self) {
        println!("file data source write")
    }

    fn read(&self) {
        println!("file data source read")
    }
}
