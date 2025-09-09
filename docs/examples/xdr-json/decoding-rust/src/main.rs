use stellar_xdr::curr::{TransactionEnvelope, ScVal, LedgerKey, WriteXdr, ReadXdr, Limits};

fn main() {
    // Create a TransactionEnvelope
    let transaction_envelope_json = r#"{
      "tx": {
        "tx": {
          "source_account": "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
          "fee": 100,
          "seq_num": 1,
          "cond": "none",
          "memo": "none",
          "operations": [
            {
              "source_account": null,
              "body": {
                "bump_sequence": { "bump_to": 2 }
              }
            }
          ],
          "ext": "v0"
        },
        "signatures": []
      }
    }"#;
    let transaction_envelope: TransactionEnvelope = serde_json::from_str(transaction_envelope_json).unwrap();
    let transaction_envelope_xdr = transaction_envelope.to_xdr_base64(Limits::none()).unwrap();
    println!("Encoded TransactionEnvelope: {}", transaction_envelope_xdr);
    let decoded_transaction_envelope = TransactionEnvelope::from_xdr_base64(&transaction_envelope_xdr, Limits::none()).unwrap();
    let decoded_json = serde_json::to_string(&decoded_transaction_envelope).unwrap();
    println!("Decoded TransactionEnvelope: {}", decoded_json);

    println!();

    // Create an ScVal
    let sc_val_json = r#"{"u32": 42}"#;
    let sc_val: ScVal = serde_json::from_str(sc_val_json).unwrap();
    let sc_val_xdr = sc_val.to_xdr_base64(Limits::none()).unwrap();
    println!("Encoded ScVal: {}", sc_val_xdr);
    let decoded_sc_val = ScVal::from_xdr_base64(&sc_val_xdr, Limits::none()).unwrap();
    let decoded_sc_val_json = serde_json::to_string(&decoded_sc_val).unwrap();
    println!("Decoded ScVal: {}", decoded_sc_val_json);

    println!();

    // Create a LedgerKey
    let ledger_key_json = r#"{"account": {"account_id": "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"}}"#;
    let ledger_key: LedgerKey = serde_json::from_str(ledger_key_json).unwrap();
    let ledger_key_xdr = ledger_key.to_xdr_base64(Limits::none()).unwrap();
    println!("Encoded LedgerKey: {}", ledger_key_xdr);
    let decoded_ledger_key = LedgerKey::from_xdr_base64(&ledger_key_xdr, Limits::none()).unwrap();
    let decoded_ledger_key_json = serde_json::to_string(&decoded_ledger_key).unwrap();
    println!("Decoded LedgerKey: {}", decoded_ledger_key_json);
}
