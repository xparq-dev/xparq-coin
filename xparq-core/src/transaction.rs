use secp256k1::{PublicKey, ecdsa::Signature};
use sha2::{Sha256, Digest};

/// Transaction type distinguishes regular transactions from mining rewards
#[derive(Clone, Debug)]
pub enum TransactionType {
    /// Regular transaction: sender -> receiver (signature stored separately)
    Regular {
        sender: PublicKey,
    },
    /// Coinbase transaction: mining reward (no sender, no signature required)
    Coinbase {
        miner_address: String,
    },
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub tx_type: TransactionType,
    pub receiver: String,
    pub amount: u64,
    pub signature: Option<Signature>,
}

impl Transaction {
    /// Create a regular transaction (unsigned)
    pub fn new(sender: PublicKey, receiver: String, amount: u64) -> Self {
        Transaction {
            tx_type: TransactionType::Regular { sender },
            receiver,
            amount,
            signature: None,
        }
    }

    /// Create a coinbase transaction (mining reward)
    pub fn coinbase(miner_address: String, reward: u64) -> Self {
        Transaction {
            tx_type: TransactionType::Coinbase {
                miner_address: miner_address.clone(),
            },
            receiver: miner_address,
            amount: reward,
            signature: None,
        }
    }

    /// Get sender address if this is a regular transaction
    pub fn sender(&self) -> Option<&PublicKey> {
        match &self.tx_type {
            TransactionType::Regular { sender } => Some(sender),
            TransactionType::Coinbase { .. } => None,
        }
    }

    /// Check if this is a coinbase transaction
    pub fn is_coinbase(&self) -> bool {
        matches!(self.tx_type, TransactionType::Coinbase { .. })
    }

    /// Set signature for regular transaction
    pub fn set_signature(&mut self, signature: Signature) {
        self.signature = Some(signature);
    }

    /// Get signature reference if available
    pub fn get_signature(&self) -> Option<&Signature> {
        self.signature.as_ref()
    }

    /// Calculate hash for signing (only includes transaction data, not signature)
    pub fn hash_bytes(&self) -> [u8; 32] {
        let data = match &self.tx_type {
            TransactionType::Regular { sender, .. } => {
                format!("{:?}{}{}", sender, self.receiver, self.amount)
            }
            TransactionType::Coinbase { miner_address } => {
                format!("COINBASE_{}{}{}", miner_address, self.receiver, self.amount)
            }
        };
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}