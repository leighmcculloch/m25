use std::{fs, rc::Rc};

use soroban_env_host::{
    Host, LedgerInfo,
    budget::Budget,
    storage::{SnapshotSource, Storage},
    xdr,
};

fn main() {
    let storage = Storage::with_recording_footprint(Rc::new(SnapshotSourceImpl));
    let budget = Budget::default();
    let host = Host::with_storage_and_budget(storage, budget);

    // Set some state for the host with basic values. When using the host in a production
    // application, appropriate values for each field should be set.
    host.set_ledger_info(LedgerInfo {
        protocol_version: 23,
        sequence_number: 0,
        timestamp: 0,
        network_id: [0; 32],
        base_reserve: 0,
        min_persistent_entry_ttl: 4096,
        min_temp_entry_ttl: 16,
        max_entry_ttl: 6_312_000,
    })
    .unwrap();
    host.set_base_prng_seed([0; 32]).unwrap();

    // Disable auth.
    host.switch_to_recording_auth(true).unwrap();

    // Load the contract.wasm and deploy it.
    let path = "../../contract/contract.wasm";
    eprintln!("Loading spec from wasm {path}.");
    let wasm = fs::read(path).unwrap();

    // Upload the contract.wasm.
    let result = host
        .invoke_function(xdr::HostFunction::UploadContractWasm(
            wasm.try_into().unwrap(),
        ))
        .unwrap();
    let xdr::ScVal::Bytes(xdr::ScBytes(wasm_hash)) = result else {
        panic!("expected result to be a bytes")
    };
    eprintln!("Uploaded wasm with hash {}.", hex::encode(&wasm_hash));

    // Deploy a contract using the contract.wasm.
    let result = host
        .invoke_function(xdr::HostFunction::CreateContractV2(
            xdr::CreateContractArgsV2 {
                contract_id_preimage: xdr::ContractIdPreimage::Address(
                    xdr::ContractIdPreimageFromAddress {
                        address: xdr::ScAddress::Contract(xdr::ContractId(xdr::Hash([0; 32]))),
                        salt: xdr::Uint256([0; 32]),
                    },
                ),
                executable: xdr::ContractExecutable::Wasm(xdr::Hash(wasm_hash.try_into().unwrap())),
                constructor_args: [].try_into().unwrap(),
            },
        ))
        .unwrap();
    let xdr::ScVal::Address(contract_address) = result else {
        panic!("expected result to be an address")
    };
    eprintln!("Deployed contract at address {contract_address}.");

    // Call the hello function.
    let result = host
        .invoke_function(xdr::HostFunction::InvokeContract(xdr::InvokeContractArgs {
            contract_address: contract_address.clone(),
            function_name: xdr::ScSymbol("hello".try_into().unwrap()),
            args: [xdr::ScVal::String(xdr::ScString(
                "test".try_into().unwrap(),
            ))]
            .try_into()
            .unwrap(),
        }))
        .unwrap();
    eprintln!("Called {contract_address}.hello(\"test\") -> {:?}", result);

    // Extract the event stream.
    eprintln!("Events:");
    let events = host.get_events().unwrap();
    for e in events.0.iter().map(|e| &e.event) {
        // Print each event using XDR-JSON.
        eprintln!("{}", serde_json::to_string(&e).unwrap());
    }
}

struct SnapshotSourceImpl;
impl SnapshotSource for SnapshotSourceImpl {
    fn get(
        &self,
        _key: &Rc<xdr::LedgerKey>,
    ) -> Result<Option<(Rc<xdr::LedgerEntry>, Option<u32>)>, soroban_env_host::HostError> {
        // This snapshot source has no data, so it will only work with contracts that do not need
        // any existing data. Fill this out, retrieving contract data from somewhere like the RPC,
        // or a local source.
        Ok(None)
    }
}
