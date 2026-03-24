#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Balance(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreasuryBalance {
    pub available_balance: i128,
    pub reserved_balance: i128,
}

#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) {
        admin.require_auth();
        if amount <= 0 {
            panic!("amount must be positive");
        }
        
        let key = DataKey::Balance(to.clone());
        let mut balance = env.storage().persistent().get::<_, TreasuryBalance>(&key).unwrap_or(TreasuryBalance {
            available_balance: 0,
            reserved_balance: 0,
        });

        balance.available_balance = balance.available_balance.checked_add(amount).expect("overflow");
        
        env.storage().persistent().set(&key, &balance);
    }

    pub fn burn(env: Env, admin: Address, from: Address, amount: i128) {
        admin.require_auth();
        if amount <= 0 {
            panic!("amount must be positive");
        }

        let key = DataKey::Balance(from.clone());
        let mut balance = env.storage().persistent().get::<_, TreasuryBalance>(&key).unwrap_or(TreasuryBalance {
            available_balance: 0,
            reserved_balance: 0,
        });

        balance.available_balance = balance.available_balance.checked_sub(amount).expect("underflow");
        
        env.storage().persistent().set(&key, &balance);
    }

    pub fn get_balance(env: Env, id: Address) -> TreasuryBalance {
        let key = DataKey::Balance(id);
        env.storage().persistent().get::<_, TreasuryBalance>(&key).unwrap_or(TreasuryBalance {
            available_balance: 0,
            reserved_balance: 0,
        })
    }
}
