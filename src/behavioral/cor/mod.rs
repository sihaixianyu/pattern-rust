pub mod department;
pub mod patient;

#[cfg(test)]
mod tests {
    use super::department::{Cashier, Department, Doctor, Medical, Reception};
    use super::patient::Patient;

    #[test]
    fn test_cor() {
        let mut reception = Reception::default();
        let doctor = Doctor::default();
        let medical = Medical::default();
        let cashier = Cashier::default();

        reception
            .set_next(Box::new(doctor))
            .set_next(Box::new(medical))
            .set_next(Box::new(cashier));

        let mut patient = Patient::new("Nagato".into());
        reception.execute(&mut patient);
    }
}
