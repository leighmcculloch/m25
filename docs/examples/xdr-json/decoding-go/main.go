package main

import (
	"encoding/base64"
	"fmt"
	"log"

	xdrjson "github.com/stellar/go-stellar-xdr-json/xdrjson"
)

func main() {
	// Create a TransactionEnvelope
	transactionEnvelopeXdrStr := "AAAAAgAAAAA/DDS/k60NmXHQTMyQ9wVRHIOKrZc0pKL7DXoD/H/omgAAAGQAAAAAAAAAAQAAAAAAAAAAAAAAAQAAAAAAAAALAAAAAAAAAAIAAAAAAAAAAA=="
	fmt.Printf("Encoded TransactionEnvelope: %s\n", transactionEnvelopeXdrStr)
	transactionEnvelopeXdr, err := base64.StdEncoding.DecodeString(transactionEnvelopeXdrStr)
	if err != nil {
		log.Fatal(err)
	}
	decodedTransactionEnvelope, err := xdrjson.Decode(xdrjson.TransactionEnvelope, transactionEnvelopeXdr)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Decoded TransactionEnvelope: %s\n", decodedTransactionEnvelope)

	fmt.Println()

	// Create an ScVal
	scValXdrStr := "AAAAAwAAACo="
	fmt.Printf("Encoded ScVal: %s\n", scValXdrStr)
	scValXdr, err := base64.StdEncoding.DecodeString(scValXdrStr)
	if err != nil {
		log.Fatal(err)
	}
	decodedScVal, err := xdrjson.Decode(xdrjson.ScVal, scValXdr)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Decoded ScVal: %s\n", decodedScVal)

	fmt.Println()

	// Create a LedgerKey
	ledgerKeyXdrStr := "AAAAAAAAAAA/DDS/k60NmXHQTMyQ9wVRHIOKrZc0pKL7DXoD/H/omg=="
	fmt.Printf("Encoded LedgerKey: %s\n", ledgerKeyXdrStr)
	ledgerKeyXdr, err := base64.StdEncoding.DecodeString(ledgerKeyXdrStr)
	if err != nil {
		log.Fatal(err)
	}
	decodedLedgerKey, err := xdrjson.Decode(xdrjson.LedgerKey, ledgerKeyXdr)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Decoded LedgerKey: %s\n", decodedLedgerKey)
}
