use crate::pattern::decorator::data_source::DataSource;
use crate::pattern::decorator::data_source_decorator::DataSourceDecorator;

pub struct CompressionDataSource {
    wrappee: Box<dyn DataSource>
}

impl DataSource for CompressionDataSource {
    fn write(&self) {
        println!("compression write");
        self.wrappee.write();
    }

    fn read(&self) {
        println!("compression read");
        self.wrappee.read();
    }
}

impl DataSourceDecorator for CompressionDataSource {
    fn wrap(data_source: impl DataSource + 'static) -> Self {
        Self {
            wrappee: Box::new(data_source),
        }
    }
}
