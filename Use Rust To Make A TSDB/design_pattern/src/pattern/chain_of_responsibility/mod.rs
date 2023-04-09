use crate::pattern::chain_of_responsibility::cashier::Cashier;
use crate::pattern::chain_of_responsibility::department::Department;
use crate::pattern::chain_of_responsibility::doctor::Doctor;
use crate::pattern::chain_of_responsibility::medical::Medical;
use crate::pattern::chain_of_responsibility::patient::Patient;
use crate::pattern::chain_of_responsibility::reception::Reception;

mod patient;
mod department;
mod reception;
mod doctor;
mod medical;

#[test]
fn test_chain_of_responsibility() {
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "xxx".to_string()
    };

    reception.execute(&mut patient);
    println!("success");
}