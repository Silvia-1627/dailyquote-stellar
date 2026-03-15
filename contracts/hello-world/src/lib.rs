#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String, symbol_short, Symbol};

const QUOTE_KEY: Symbol = symbol_short!("QUOTE");

#[contract]
pub struct DailyQuoteContract;

#[contractimpl]
impl DailyQuoteContract {

    // Set the daily quote
    pub fn set_quote(env: Env, quote: String) {
        env.storage().instance().set(&QUOTE_KEY, &quote);
    }

    // Get the daily quote
    pub fn get_quote(env: Env) -> String {
        env.storage()
            .instance()
            .get(&QUOTE_KEY)
            .unwrap_or(String::from_str(&env, "No quote set yet"))
    }
}