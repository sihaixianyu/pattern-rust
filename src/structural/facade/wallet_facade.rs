use super::{
    account::{Account},
    ledger::Ledger,
    notification::Notification,
    security_code::{SecurityCode},
    wallet::Wallet,
};

pub struct WalletFacade {
    account: Account,
    wallet: Wallet,
    security_code: SecurityCode,
    notification: Notification,
    ledger: Ledger,
}

impl WalletFacade {
    pub fn new(account: String, code: u32) -> Self {
        println!("Starting create account...");

        let facade = Self {
            account: Account::new(account),
            wallet: Wallet::new(),
            security_code: SecurityCode::new(code),
            notification: Notification,
            ledger: Ledger,
        };

        println!("Account created!");
        facade
    }

    pub fn add_money_to_wallet(
        &mut self,
        account: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting to add money to wallet");
        self.account.check(account)?;
        self.security_code.check(security_code)?;
        self.wallet.credit_balance(amount);
        self.notification.send_credit_notification();
        self.ledger.make_entry(account, "credit".to_string(), amount);

        Ok(())
    }

    pub fn deduct_money_to_wallet(
        &mut self,
        account: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting to add money to wallet");
        self.account.check(account)?;
        self.security_code.check(security_code)?;
        self.wallet.debit_balance(amount);
        self.notification.send_debit_notification();
        self.ledger.make_entry(account, "debit".to_string(), amount);

        Ok(())
    }
}
