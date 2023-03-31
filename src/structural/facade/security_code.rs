pub struct SecurityCode {
    code: u32,
}

impl SecurityCode {
    pub fn new(code: u32) -> SecurityCode {
        Self { code }
    }

    pub fn check(&self, code: u32) -> Result<(), String> {
        if self.code != code {
            return Err("Security code is incorrect!".to_string());
        }

        println!("Security code verified!");
        Ok(())
    }
}
