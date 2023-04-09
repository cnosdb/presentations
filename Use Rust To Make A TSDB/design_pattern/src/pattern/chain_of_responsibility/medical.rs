use crate::pattern::chain_of_responsibility::department::Department;
use crate::pattern::chain_of_responsibility::patient::Patient;

pub struct Medical {
    next: Option<Box<dyn Department>>,
}

impl Medical {
    pub fn new(next: impl Department + 'static) -> Self {
        Self {
            next: Some(Box::new(next)),
        }
    }
}

impl Department for Medical {
    fn handle(&mut self, patient: &mut Patient) {
        println!("Medical giving medicine to a patient {}", patient.name);
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}