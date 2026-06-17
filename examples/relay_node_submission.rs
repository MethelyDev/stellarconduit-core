use stellarconduit_core::message::types::TransactionEnvelope;
use stellarconduit_core::relay::RelayNode;
use stellarconduit_core::relay::StellarRpcClient;

struct ExampleRpcClient;

impl StellarRpcClient for ExampleRpcClient {
    fn submit_transaction(&self, tx_xdr: &str) -> Result<String, String> {
        println!(
            "Submitting transaction: {}...",
            &tx_xdr[..20.min(tx_xdr.len())]
        );
        // In production this would call the Stellar RPC endpoint
        Ok("example_tx_hash".to_string())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let rpc_client = Box::new(ExampleRpcClient);
    let mut relay = RelayNode::new(1000, rpc_client);

    let envelope = TransactionEnvelope {
        message_id: [1u8; 32],
        origin_pubkey: [2u8; 32],
        tx_xdr: "AAAAAgAAAADZ/7+9/7+9/7+9EXAMPLE_XDR".to_string(),
        ttl_hops: 10,
        timestamp: 1672531200,
        signature: [3u8; 64],
    };

    println!("Processing transaction envelope...");
    println!("Origin: {:?}", hex::encode(&envelope.origin_pubkey[..8]));
    println!("Timestamp: {}", envelope.timestamp);

    match relay.process_envelope(&envelope) {
        Ok(tx_hash) => {
            println!("✓ Transaction submitted successfully!");
            println!("Transaction hash: {}", tx_hash);
        }
        Err(e) => {
            eprintln!("✗ Failed to process envelope: {}", e);
        }
    }

    Ok(())
}
