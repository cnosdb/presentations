
pub trait DataSource {
    fn write(&self);
    fn read(&self);
}
