use super::patient::Patient;

pub trait Department {
    // This function is designed for acquisition of next Department.
    fn next(&mut self) -> &mut Option<Box<dyn Department>>;
    fn set_next(&mut self, next: Box<dyn Department>) -> &mut Box<dyn Department>;
    fn handle(&mut self, patient: &mut Patient);

    fn execute(&mut self, patient: &mut Patient) {
        self.handle(patient);

        if let Some(next) = &mut self.next() {
            return next.execute(patient);
        }
    }
}

#[derive(Default)]
pub struct Reception {
    next: Option<Box<dyn Department>>,
}

impl Department for Reception {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.registration_done {
            println!("Patient finished registration.");
            return;
        }

        println!("Patient: {} is registering...", patient.name);
        patient.registration_done = true;
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }

    fn set_next(&mut self, next: Box<dyn Department>) -> &mut Box<dyn Department> {
        self.next = Some(next);
        self.next.as_mut().unwrap()
    }
}

#[derive(Default)]
pub struct Doctor {
    next: Option<Box<dyn Department>>,
}

impl Department for Doctor {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.checkup_done {
            println!("Doctor has finished consultation.");
            return;
        }

        println!("Doctor is checkupping ...");
        patient.checkup_done = true;
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }

    fn set_next(&mut self, next: Box<dyn Department>) -> &mut Box<dyn Department> {
        self.next = Some(next);
        self.next.as_mut().unwrap()
    }
}

#[derive(Default)]
pub struct Medical {
    next: Option<Box<dyn Department>>,
}

impl Department for Medical {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.medicine_done {
            println!("Patient got medicine.");
            return;
        }

        println!("Pharmacist is making up a prescription...");
        patient.medicine_done = true;
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }

    fn set_next(&mut self, next: Box<dyn Department>) -> &mut Box<dyn Department> {
        self.next = Some(next);
        self.next.as_mut().unwrap()
    }
}

#[derive(Default)]
pub struct Cashier {
    next: Option<Box<dyn Department>>,
}

impl Department for Cashier {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.payment_done {
            println!("Patient finished payment.");
            return;
        }

        println!("Payment is under way...");
        patient.payment_done = true;
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }

    fn set_next(&mut self, next: Box<dyn Department>) -> &mut Box<dyn Department> {
        self.next = Some(next);
        self.next.as_mut().unwrap()
    }
}
