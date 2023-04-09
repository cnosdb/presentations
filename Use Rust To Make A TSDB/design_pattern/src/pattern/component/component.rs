// 抽象类组件
pub trait Component {
    fn search(&self, keyword: &str);
    fn name(&self) -> &'static str;
}

pub trait ComponentIterator {

}