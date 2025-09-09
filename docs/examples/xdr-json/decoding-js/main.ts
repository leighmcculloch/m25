import init, { decode, encode } from "@stellar/stellar-xdr-json";
await init();

// Create a TransactionEnvelope
const transactionEnvelopeJson = JSON.stringify({
  tx: {
    tx: {
      source_account:
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
      fee: 100,
      seq_num: "1",
      cond: "none",
      memo: "none",
      operations: [
        {
          source_account: null,
          body: {
            bump_sequence: { bump_to: "2" },
          },
        },
      ],
      ext: "v0",
    },
    signatures: [],
  },
});
const transactionEnvelopeXdr = encode(
  "TransactionEnvelope",
  transactionEnvelopeJson,
);
console.log("Encoded TransactionEnvelope:", transactionEnvelopeXdr);
const decodedTransactionEnvelope = decode(
  "TransactionEnvelope",
  transactionEnvelopeXdr,
);
console.log("Decoded TransactionEnvelope:", decodedTransactionEnvelope);

console.log();

// Create an ScVal
const scValJson = JSON.stringify({ u32: 42 });
const scValXdr = encode("ScVal", scValJson);
console.log("Encoded ScVal:", scValXdr);
const decodedScVal = decode("ScVal", scValXdr);
console.log("Decoded ScVal:", decodedScVal);

console.log();

// Create a LedgerKey
const ledgerKeyJson = JSON.stringify({
  account: {
    account_id: "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
  },
});
const ledgerKeyXdr = encode("LedgerKey", ledgerKeyJson);
console.log("Encoded LedgerKey:", ledgerKeyXdr);
const decodedLedgerKey = decode("LedgerKey", ledgerKeyXdr);
console.log("Decoded LedgerKey:", decodedLedgerKey);
