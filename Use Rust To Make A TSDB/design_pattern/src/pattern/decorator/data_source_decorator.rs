use crate::pattern::decorator::data_source::DataSource;

pub trait DataSourceDecorator: DataSource {
    fn wrap(data_source: impl DataSource + 'static) -> Self;
}
