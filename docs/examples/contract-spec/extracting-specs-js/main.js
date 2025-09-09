import { contract } from "@stellar/stellar-sdk";

const path = "../../../contract/contract.wasm";
console.log(`Loading spec from wasm ${path}.`);
const wasmBytes = await Deno.readFile(path);
const specs = await contract.Spec.fromWasm(wasmBytes);
const funcs = specs.funcs();
const funcNames = funcs.map((f) => f.name().toString());

console.log("Functions in contract:");
console.log(funcNames);
