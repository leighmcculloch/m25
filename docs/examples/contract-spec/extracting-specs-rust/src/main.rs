use std::fs;

use stellar_xdr::curr::ScSpecEntry;

fn main() {
    let path = "../../../contract/contract.wasm";
    eprintln!("Loading spec from wasm {path}.");
    let wasm = fs::read(path).unwrap();
    let specs = soroban_spec::read::from_wasm(&wasm).unwrap();
    eprintln!("Functions in contract:");
    for entry in specs {
        match entry {
            ScSpecEntry::FunctionV0(func) => {
                println!("{}", func.name.to_string());
            }
            _ => (),
        }
    }
}
