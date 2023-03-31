pub struct Ledger;

impl Ledger {
    pub fn make_entry(&mut self, account: &String, txn_type: String, amount: u32) {
        println!(
            "Make ledger entry for account {} with transaction type {} and amount {}",
            account, txn_type, amount
        );
    }
}
