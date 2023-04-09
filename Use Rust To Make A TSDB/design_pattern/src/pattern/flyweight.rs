use std::collections::HashMap;
use std::sync::Arc;

pub struct BigObject {
    data: [u64; 1000]
}

impl Default for BigObject {
    fn default() -> Self {
        Self {
            data: [0; 1000]
        }
    }
}

#[derive(Default)]
pub struct FlyweightFactory {
    objects: HashMap<String, Arc<BigObject>>
}

impl FlyweightFactory {
    pub fn get(&self, name: &str) -> Option<Arc<BigObject>> {
        self.objects.get(name).cloned()
    }

    pub fn create<F: FnOnce() -> BigObject>(&mut self, name: String, create_object: F) -> Arc<BigObject> {
        let res = Arc::new(create_object());
        self.objects.insert(name, res.clone());
        res
    }
}

#[test]
fn test_flyweight() {
    let mut factory = FlyweightFactory::default();
    factory.create("object1".to_string(), BigObject::default);

    let object = match factory.get("object1") {
        Some(o) => o,
        None => factory.create("object1".to_string(), BigObject::default)
    };


}
