#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, Env, String};

#[contract]
pub struct Contract;

/// Indicates that hello has been said to the name.
#[contractevent]
pub struct Hello {
    /// The name being hello'd.
    pub name: String,
}

/// Indicates that bye has been said to the name.
#[contractevent]
pub struct Bye {
    /// The name being bye'd.
    pub name: String,
}

#[contractimpl]
impl Contract {
    /// Function hello publishes a [Hello] event with the given name.
    pub fn hello(env: &Env, name: String) {
        Hello { name }.publish(&env);
    }

    /// Function hello publishes a [Bye] event with the given name.
    pub fn bye(env: &Env, name: String) {
        Bye { name }.publish(&env);
    }
}

mod test;
