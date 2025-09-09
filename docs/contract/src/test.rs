#![cfg(test)]

use super::*;
use soroban_sdk::{map, symbol_short, testutils::Events, vec, Env, IntoVal, String};

#[test]
fn test_hello() {
    // Setup the environment.
    let env = Env::default();

    // Deploy the contract.
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Call the 'hello' function, passing in the name.
    client.hello(&String::from_str(&env, "friend"));

    // Assert that the Hello event was published.
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            // Hello event.
            (
                contract_id.clone(),
                // Event topics.
                vec![&env, symbol_short!("hello").into_val(&env)],
                // Event data.
                map![
                    &env,
                    (symbol_short!("name"), String::from_str(&env, "friend"))
                ]
                .into_val(&env)
            )
        ]
    )
}

#[test]
fn test_bye() {
    // Setup the environment.
    let env = Env::default();

    // Deploy the contract.
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Call the 'bye' function, passing in the name.
    client.bye(&String::from_str(&env, "friend"));

    // Assert that the Hello event was published.
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            // Bye event.
            (
                contract_id.clone(),
                // Event topics.
                vec![&env, symbol_short!("bye").into_val(&env)],
                // Event data.
                map![
                    &env,
                    (symbol_short!("name"), String::from_str(&env, "friend"))
                ]
                .into_val(&env)
            )
        ]
    )
}
