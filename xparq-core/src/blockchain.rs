use std::collections::{HashMap, HashSet};
use crate::block::Block;
use crate::transaction::{Transaction, TransactionType};
use crate::crypto::verify_signature;

/// Configuration for blockchain parameters
const BLOCK_REWARD: u64 = 10; // XPARQ per block

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub balances: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain {
            chain: vec![],
            difficulty: 3,
            balances: HashMap::new(),
        };

        // Genesis block: no transactions
        bc.chain.push(Block::new(
            0,
            vec![],
            "0".to_string(),
            "GENESIS".to_string(),
        ));
        bc
    }

    /// Add a new block with transactions
    /// 
    /// Process:
    /// 1. Create coinbase transaction for miner
    /// 2. Apply coinbase first (miner receives reward for this block)
    /// 3. Validate all user transactions against updated state
    /// 4. Apply valid transactions atomically
    /// 5. Mine the block (PoW)
    /// 6. Add to chain
    pub fn add_block(&mut self, txs: Vec<Transaction>, miner_address: String) -> Result<(), String> {
        let prev = self.chain.last().unwrap();

        // 1. Create coinbase transaction for miner
        let coinbase = Transaction::coinbase(miner_address.clone(), BLOCK_REWARD);

        // 2. Apply coinbase FIRST (miner gets reward before validating their transactions)
        let miner_balance = self.balances.entry(miner_address.clone()).or_insert(0);
        *miner_balance += BLOCK_REWARD;

        // 3. Validate all user transactions against updated state (including mining reward)
        for tx in &txs {
            self.validate_transaction(tx)?;
        }

        // 4. Prevent double-spending within block (track addresses)
        let mut applied_senders = HashSet::new();
        for tx in &txs {
            if let Some(sender_pubkey) = tx.sender() {
                let sender_addr = self.pubkey_to_address(sender_pubkey);
                if !applied_senders.insert(sender_addr.clone()) {
                    return Err("Double-spending attempt within block".to_string());
                }
            }
        }

        // 5. Create and mine block (with all transactions: coinbase + user txs)
        let mut all_txs = vec![coinbase];
        all_txs.extend(txs.clone());

        let mut block = Block::new(
            self.chain.len() as u64,
            all_txs,
            prev.hash.clone(),
            miner_address,
        );

        block.mine(self.difficulty);

        // 6. Apply user transactions (coinbase already applied above)
        for tx in &txs {
            match &tx.tx_type {
                TransactionType::Regular { sender, .. } => {
                    let sender_addr = self.pubkey_to_address(sender);
                    let balance = self.balances.entry(sender_addr).or_insert(0);
                    *balance -= tx.amount;

                    let receiver_balance = self.balances.entry(tx.receiver.clone()).or_insert(0);
                    *receiver_balance += tx.amount;
                }
                TransactionType::Coinbase { .. } => {
                    // Already handled
                }
            }
        }

        self.chain.push(block);
        Ok(())
    }

    /// Validate a single transaction
    /// 
    /// Checks:
    /// - Regular txs: must have valid signature
    /// - Regular txs: amount > 0
    /// - Regular txs: sender has sufficient balance
    /// - Coinbase txs: are always valid (controlled by us)
    /// Validate a single transaction
    /// 
    /// Checks:
    /// - Regular txs: must have valid signature
    /// - Regular txs: amount > 0
    /// - Regular txs: sender has sufficient balance
    /// - Coinbase txs: are always valid (controlled by us)
    fn validate_transaction(&self, tx: &Transaction) -> Result<(), String> {
        match &tx.tx_type {
            TransactionType::Regular { sender } => {
                // Amount must be positive
                if tx.amount == 0 {
                    return Err("Transaction amount must be > 0".to_string());
                }

                // Signature must exist
                let signature = match tx.get_signature() {
                    Some(sig) => sig,
                    None => return Err("Transaction not signed".to_string()),
                };

                // Verify signature
                let hash = tx.hash_bytes();
                if !verify_signature(sender, &hash, signature) {
                    return Err("Invalid transaction signature".to_string());
                }

                // Check sender has sufficient balance
                let sender_addr = self.pubkey_to_address(sender);
                let sender_balance = self.balances.get(&sender_addr).copied().unwrap_or(0);
                if sender_balance < tx.amount {
                    return Err(format!(
                        "Insufficient balance: {} has {}, needs {}",
                        sender_addr, sender_balance, tx.amount
                    ));
                }

                Ok(())
            }
            TransactionType::Coinbase { .. } => {
                // Coinbase transactions are always valid (created by us)
                Ok(())
            }
        }
    }

    /// Convert public key to address (SHA256 hash)
    fn pubkey_to_address(&self, pubkey: &secp256k1::PublicKey) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(pubkey.serialize());
        hex::encode(hasher.finalize())
    }

    /// Get balance of an address
    pub fn get_balance(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }
}