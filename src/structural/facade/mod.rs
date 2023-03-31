pub mod account;
pub mod ledger;
pub mod notification;
pub mod security_code;
pub mod wallet;
pub mod wallet_facade;

#[cfg(test)]
mod tests {
    use crate::structural::facade::wallet_facade::WalletFacade;

    #[test]
    fn test_facade() -> Result<(), String> {
        let mut wallet = WalletFacade::new("abc".to_string(), 1234);
        println!();

        wallet.add_money_to_wallet(&"abc".into(), 1234, 10)?;
        println!();

        wallet.deduct_money_to_wallet(&"abc".into(), 1234, 5)
    }
}
