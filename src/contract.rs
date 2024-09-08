use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, Vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceInfo {
    pub address: Address,
    pub balance: i128,
}

#[contract]
pub struct BalanceContract;

#[contractimpl]
impl BalanceContract {
    pub fn get_balance(env: &Env, address: Address) -> Result<BalanceInfo, String> {
        let balance_symbol = Symbol::short("BALANCE");
        let key = (address.clone(), balance_symbol.clone());


        match env.storage().get::<(Address, Symbol), i128>(&key) {
            Some(balance) => Ok(BalanceInfo { address, balance }),
            None => Err(String::from("Bakiye bulunamadı")),
        }
    }
}

#[contract]
pub struct MultiTransfer;

#[contractimpl]
impl MultiTransfer {
    pub fn transfer_to_multiple(
        env: &Env,
        from: Address,
        recipients: Vec<Address>,
        amounts: Vec<i128>,
    ) {
        assert!(recipients.len() == amounts.len(), "Alıcılar ve miktarlar eşleşmiyor.");

        for i in 0..recipients.len() {
            let to = &recipients[i];
            let amount = amounts[i];


            env.events().publish((from.clone(), to.clone()), Symbol::short("transfer"));


        }
    }
}

#[contract]
pub struct ScheduledPayment;

#[contractimpl]
impl ScheduledPayment {
    pub fn schedule_payment(
        env: &Env,
        from: Address,
        to: Address,
        amount: i128,
        interval: u64,
    ) {
        let symbol = Symbol::short("scheduled_payment");


        let participants = (from.clone(), to.clone());


        env.events().publish(participants, symbol);


        let from_cloned = from.clone();
        let to_cloned = to.clone();
        let amount_cloned = amount;


        env.set_timeout(interval, move || {

            env.events().publish((from_cloned.clone(), to_cloned.clone()), symbol);
        });
    }
}
