use crate::pattern::chain_of_responsibility::patient::Patient;

pub trait Department {
    fn execute(&mut self, patient: &mut Patient) {
        self.handle(patient);
        self.next().as_mut().map( |mut n| {
            n.execute(patient)
        });
    }

    fn handle(&mut self, patient: &mut Patient);
    fn next(&mut self) -> &mut Option<Box<dyn Department>>;
}