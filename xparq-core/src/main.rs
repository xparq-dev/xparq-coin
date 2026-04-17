mod crypto;
mod transaction;
mod block;
mod blockchain;

use crypto::Wallet;
use transaction::Transaction;
use blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new();

    // Create wallets
    let wallet_a = Wallet::new();
    let wallet_b = Wallet::new();
    let miner = Wallet::new();

    let addr_a = wallet_a.get_address();
    let addr_b = wallet_b.get_address();
    let miner_addr = miner.get_address();

    println!("=== XPARQ Blockchain Demo ===");
    println!("Wallet A: {}", addr_a);
    println!("Wallet B: {}", addr_b);
    println!("Miner:    {}", miner_addr);
    println!();

    // Block 1: Miner gets first reward (coinbase)
    println!("[Block 1] Mining genesis block...");
    match chain.add_block(vec![], miner_addr.clone()) {
        Ok(_) => {
            let mine_balance = chain.get_balance(&miner_addr);
            println!("✓ Miner balance: {} XPARQ", mine_balance);
            println!();
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Block 2: Miner sends to Wallet A
    println!("[Block 2] Miner transfers to Wallet A...");
    let mut tx1 = Transaction::new(miner.public_key, addr_a.clone(), 5);
    let hash1 = tx1.hash_bytes();
    let sig1 = miner.sign_hash(&hash1);
    tx1.set_signature(sig1);

    match chain.add_block(vec![tx1], miner_addr.clone()) {
        Ok(_) => {
            let miner_balance = chain.get_balance(&miner_addr);
            let a_balance = chain.get_balance(&addr_a);
            println!("✓ Miner balance: {} XPARQ", miner_balance);
            println!("✓ Wallet A balance: {} XPARQ", a_balance);
            println!();
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Block 3: Wallet A sends to Wallet B
    println!("[Block 3] Wallet A transfers to Wallet B...");
    let mut tx2 = Transaction::new(wallet_a.public_key, addr_b.clone(), 3);
    let hash2 = tx2.hash_bytes();
    let sig2 = wallet_a.sign_hash(&hash2);
    tx2.set_signature(sig2);

    match chain.add_block(vec![tx2], miner_addr.clone()) {
        Ok(_) => {
            let a_balance = chain.get_balance(&addr_a);
            let b_balance = chain.get_balance(&addr_b);
            println!("✓ Wallet A balance: {} XPARQ", a_balance);
            println!("✓ Wallet B balance: {} XPARQ", b_balance);
            println!();
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Try invalid transaction (insufficient balance)
    println!("[Block 4] Testing invalid transaction (insufficient balance)...");
    let mut tx3 = Transaction::new(wallet_a.public_key, addr_b.clone(), 100);
    let hash3 = tx3.hash_bytes();
    let sig3 = wallet_a.sign_hash(&hash3);
    tx3.set_signature(sig3);

    match chain.add_block(vec![tx3], miner_addr.clone()) {
        Ok(_) => println!("Should have failed!"),
        Err(e) => println!("✓ Correctly rejected: {}", e),
    }
    println!();

    // Summary
    println!("=== Final State ===");
    println!("Chain height: {} blocks", chain.chain.len());
    println!("Wallet A: {} XPARQ", chain.get_balance(&addr_a));
    println!("Wallet B: {} XPARQ", chain.get_balance(&addr_b));
    println!("Miner:    {} XPARQ", chain.get_balance(&miner_addr));
}