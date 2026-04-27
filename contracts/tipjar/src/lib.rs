#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Vec,
};

// Data structure for one tip entry stored on chain
#[contracttype]
#[derive(Clone)]
pub struct TipEntry {
    pub sender: Address,
    pub amount: i128,        // in stroops (1 XLM = 10_000_000 stroops)
    pub message: String,
    pub timestamp: u64,
}

// Storage keys
const TIPS_KEY: &str = "TIPS";
const TOTAL_KEY: &str = "TOTAL";

#[contract]
pub struct TipJarContract;

#[contractimpl]
impl TipJarContract {
    
    /// Log a tip on-chain. Called from frontend after successful XLM payment.
    /// sender: the tipper's Stellar address
    /// amount: tip amount in stroops (multiply XLM by 10_000_000)
    /// message: optional message from tipper
    pub fn log_tip(env: Env, sender: Address, amount: i128, message: String) {
        // Require the sender to authorize this call
        sender.require_auth();

        // Create the tip entry
        let tip = TipEntry {
            sender: sender.clone(),
            amount,
            message,
            timestamp: env.ledger().timestamp(),
        };

        // Load existing tips or create empty vec
        let mut tips: Vec<TipEntry> = env
            .storage()
            .persistent()
            .get(&symbol_short!("TIPS"))
            .unwrap_or(Vec::new(&env));

        // Add new tip to the list
        tips.push_back(tip);

        // Store updated tips list (90 day TTL)
        env.storage()
            .persistent()
            .set(&symbol_short!("TIPS"), &tips);

        env.storage()
            .persistent()
            .extend_ttl(&symbol_short!("TIPS"), 0, 6480000);

        // Update running total
        let current_total: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("TOTAL"))
            .unwrap_or(0i128);

        let new_total = current_total + amount;
        env.storage()
            .persistent()
            .set(&symbol_short!("TOTAL"), &new_total);
    }

    /// Get all logged tips
    pub fn get_tips(env: Env) -> Vec<TipEntry> {
        env.storage()
            .persistent()
            .get(&symbol_short!("TIPS"))
            .unwrap_or(Vec::new(&env))
    }

    /// Get total amount tipped (in stroops)
    pub fn get_total(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&symbol_short!("TOTAL"))
            .unwrap_or(0i128)
    }
}
