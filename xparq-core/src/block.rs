use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub miner_address: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        miner_address: String,
    ) -> Self {
        let timestamp = Self::now();

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            miner_address,
            nonce: 0,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    fn now() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}