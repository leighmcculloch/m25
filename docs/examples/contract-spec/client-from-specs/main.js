import { contract, Networks } from "@stellar/stellar-sdk";

// Create a client using the contract ID. The client object returned will be
// generated from the contract spec by downloading the contract using the RPC.
console.log(
  "Creating client for CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH.",
);
const client = await contract.Client.from({
  contractId: "CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH",
  networkPassphrase: Networks.PUBLIC,
  rpcUrl: "https://mainnet.sorobanrpc.com",
});

// Print all functions defined on the client
console.log("Functions on client:");
console.log(Object.getOwnPropertyNames(client));

// The client contains a function for each function in the contract spec, that
// accepts fields with the appropriate type as required by the function.
// Calling the function will run simulation through the RPC and hand back an
// AssembledTransaction that contains a result of the simulation, as well as
// events generated, and information needed to build a transaction to submit.
console.log("Result of calling 'get_factory':");
const resp = await client.get_factory();
console.log(resp.result);
