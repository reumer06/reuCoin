# reuCoin

A minimalist, educational blockchain built from scratch in Rust.

This project serves as a hands-on learning tool designed to demystify the core mechanics of cryptocurrency systems. It
features a simplified implementation of proof-of-work mining, transaction lifecycle management, and an interactive
command-line wallet.

## 📂Project Structure

```
reuCoin/
├── src/
│   ├── lib/                    
│   │   └── src/
│   │       ├── crypto.rs       
│   │       ├── types.rs        
│   │       ├── network.rs      
│   │       ├── sha256.rs       
│   │       ├── error.rs        
│   │       └── util.rs         
│   ├── node/                   
│   │   └── src/
│   │       ├── main.rs         
│   │       ├── handler.rs
│   │       └── util.rs        
│   ├── miner/                  
│   │   ├── src/
│   │   │   └── main.rs         
│   └── wallet
│       └── src/
│           ├── main.rs
│           ├── core.rs         
│           └── util.rs         
├── Cargo.toml                  
└── README.md                   
```

## ⚙️ Setup & Installation

### Prerequisites

Ensure you have the Rust toolchain installed (compiler, Cargo).

### 1. Build Source

Clone the repository and build all workspace targets in release mode:

```bash
git clone https://github.com/reumer06/reuCoin.git
cd reuCoin
cargo build --release

```

### 2. Start Node

Run the primary seed node to maintain the ledger state:

```bash
cargo run --bin node -- --port 9000 --blockchain-file ./blockchain.cbor

```

*Options:* `--port <NUM>`, `--blockchain-file <PATH>`, `[NODES...]` (connect addresses).

Multi-Node Network example:

```bash
# Node 1
cargo run --bin node -- --port 9000
# Node 2
cargo run --bin node -- --port 9001 127.0.0.1:9000

```

### 3. Start Miner (Optional)

The miner connects to an active node to pull block templates and solve the PoW puzzle. It requires a node address and
your public key file to credit rewards:

```bash
cargo run --bin miner -- --address 127.0.0.1:9000 --public-key-file ./src/miner/alice.pub.pem

```

### 4. Setup Wallet

Generate a configuration file:

```bash
cargo run --bin wallet -- generate-config --output wallet_config.toml

```

Open wallet_config.toml to manage your keys, contacts, and transaction fee rules:

```toml
my_keys = ["your_private_key.cbor"]

[[contacts]]
name = "Alice"
key = "alice.pub.pem"

[[contacts]]
name = "Bob"
key = "bob.pub.pem"

[fee_config]
fee_type = "Percent"
value = 0.1

```

### 5. Use Wallet

Run wallet to query balances and broadcast transactions to the network.

```bash
cargo run --bin wallet -- --config wallet_config.toml --node 127.0.0.1:9000

```

Commands:

    balance — Check current UTXO valuation.

    send <recipient> <amount> — Sign and broadcast a transaction.

    exit — Close the wallet application.

Session example:

```
> balance
Current balance: 0 satoshis

> send Alice 1000
Transaction sent successfully

> exit

```

## System Architecture

```text
+-----------------------+         +-----------------------+
|        Wallet         |         |         Miner         |
| - Tracks UTXO/Balance |         | - Solves Proof-of-Work|
| - Signs & Sends Tx    |         | - Fetches Templates   |
+-----------+-----------+         +-----------+-----------+
            |                                 |
            | (Transactions)                  | (Blocks/Templates)
            +----------------+----------------+
                             |
                             v
                 +-----------------------+
                 |         Node          |
                 | - Holds Chain/Mempool |
                 | - Validates Blocks    |
                 | - Saves State         |
                 +-----------+-----------+
                             |
                             | (Uses)
                             v
                 +-----------------------+
                 |     Core Library      |
                 | - crypto.rs / types.rs|
                 | - network.rs / util.rs|
                 | - sha256.rs           |
                 +-----------------------+

```

## System Parameters

From `src/lib/src/lib.rs`:

```rust
/// Initial block reward in satoshis
pub const INITIAL_REWARD: u64 = 50;

/// Blocks until reward halves
pub const HALVING_INTERVAL: u64 = 210;

/// Target block time in seconds
pub const IDEAL_BLOCK_TIME: u64 = 10;

/// Difficulty adjustment interval in blocks
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;

/// Max transactions per block
pub const BLOCK_TRANSACTION_CAP: usize = 20;

/// Max mempool age in blocks
pub const MAX_MEMPOOL_TRANSACTION_AGE: u64 = 600;
```

## 🛠️ Debugging

Monitor storage footprints and open port bindings to resolve peer connection issues:

```bash
# Check raw ledger database size
ls -lh blockchain.cbor

# Verify node is listening on target port
netstat -an | grep 9000
```

To view fine-grained pipeline state execution, launch runtime bins prefixed with the RUST_LOG environment variable:

```bash
RUST_LOG=debug cargo run --bin node

```

## 📄 License

This project is licensed under the terms of the MIT License. See the LICENSE file for details.