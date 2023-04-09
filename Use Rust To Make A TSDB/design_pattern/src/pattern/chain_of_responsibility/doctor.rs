use crate::pattern::chain_of_responsibility::department::Department;
use crate::pattern::chain_of_responsibility::patient::Patient;

#[derive(Default)]
pub struct Doctor {
    next: Option<Box<dyn Department>>
}

impl Doctor {
    pub fn new(next: impl Department + 'static) -> Self {
        Self {
            next: Some(Box::new(next))
        }
    }
}

impl Department for Doctor {
    fn handle(&mut self, patient: &mut Patient) {
        println!("Doctor checking a patient {}", patient.name);
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}
