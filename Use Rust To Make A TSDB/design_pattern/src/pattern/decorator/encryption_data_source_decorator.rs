use crate::pattern::decorator::data_source::DataSource;
use crate::pattern::decorator::data_source_decorator::DataSourceDecorator;

pub struct EncryptionDataSourceDecorator {
    wrappee: Box<dyn DataSource>
}

impl DataSource for EncryptionDataSourceDecorator {
    fn write(&self) {
        println!("encryption write");
        self.wrappee.write();
    }

    fn read(&self) {
        println!("encryption read");
        self.wrappee.read();
    }
}

impl DataSourceDecorator for EncryptionDataSourceDecorator {
    fn wrap(data_source: impl DataSource + 'static) -> Self {
        Self {
            wrappee: Box::new(data_source),
        }
    }
}