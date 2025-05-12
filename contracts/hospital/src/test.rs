// #![cfg(test)]

// use soroban_sdk::{Env, Address, testutils::Address as _};

// use crate::{HospitalContract, HospitalContractClient};

// pub struct TestSetup {
//     pub env: Env,
//     pub client: HospitalContractClient,
//     pub admin: Address,
// }

// pub fn setup() -> TestSetup {
//     let env = Env::default();

//     let contract_id = env.register_contract(None, HospitalContract);
//     let client = HospitalContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     client.initialize(&admin);

//     TestSetup { env, client, admin }
// }
