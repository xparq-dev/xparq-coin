# XPARQ COIN Protocol Specification (v0.1)

## 1. Overview

XPARQ Coin is a decentralized Proof-of-Work blockchain designed to function as a long-term store of value ("Digital Platinum").

Core principles:
- Fixed supply
- No central authority
- High security through computational cost
- Increasing scarcity over time

---

## 2. Monetary Policy

- Max Supply: 12,000,000 XPARQ
- Pre-mine: 0
- Genesis Allocation: 0
- Smallest Unit: 1 XPARQ = 100,000,000 units (like satoshi)

---

## 3. Block Parameters

- Block Time Target: 300 seconds (5 minutes)
- Block Size Limit: 1 MB (initial)
- Block Reward (initial): 10 XPARQ

---

## 4. Emission Schedule

Reward halves every 3 years (~315,360 blocks)

| Phase | Block Range | Reward |
|------|------------|--------|
| 1 | 0 – 315,360 | 10 |
| 2 | 315,360 – 630,720 | 5 |
| 3 | 630,720 – 946,080 | 2.5 |
| ... | ... | ... |

Final supply asymptotically approaches 12,000,000 XPARQ

---

## 5. Consensus Mechanism

Algorithm: Proof-of-Work (PoW)

Hashing:
- Primary: SHA-256
- Optional future: Hybrid memory-hard layer

Mining condition:
Hash(block_header) < target

---

## 6. Difficulty Adjustment

- Adjustment Interval: Every block
- Target Block Time: 300 seconds

Formula (simplified):

new_difficulty = old_difficulty * (actual_time / target_time)

Constraints:
- Max adjustment per block: ±10%
- Prevent extreme swings

---

## 7. Block Structure

Block Header:
- version
- previous_block_hash
- merkle_root
- timestamp
- difficulty_target
- nonce

Block Body:
- transaction_count
- transactions[]

---

## 8. Transaction Model

Structure:
- sender_public_key
- recipient_address
- amount
- fee
- signature

Validation:
- Signature must be valid
- Balance must be sufficient
- No double-spend

---

## 9. Cryptography

- Hash Function: SHA-256
- Signature Algorithm: ECDSA (secp256k1)

Future upgrade path:
- Ed25519 (optional via soft fork)

---

## 10. Network Layer

- Peer-to-peer architecture
- Gossip protocol for transaction propagation
- Full nodes validate entire chain

---

## 11. Genesis Block

Timestamp: TBD  
Message: "XPARQ begins — scarcity is law"  
Initial difficulty: fixed low value  

---

## 12. Security Model

Security derives from:
- Computational cost (PoW)
- Network decentralization
- Economic incentives

Attack resistance:
- 51% attack requires majority hash power
- Cost scales with network size

---

## 13. AI Integration (Non-consensus)

AI is NOT part of consensus.

Allowed uses:
- Node performance optimization
- Network anomaly detection
- Monitoring tools

AI cannot:
- Validate blocks
- Change consensus rules
- Control supply

---

## 14. Governance

- No central authority
- Protocol changes require:
  - Node consensus
  - Hard fork or soft fork

---

## 15. Philosophy

XPARQ is designed not for speed, but for permanence.

Scarcity is enforced by code.
Security is enforced by cost.
Value is determined by time.
