# Quick Start Guide - reuCoin

Get up and running with reuCoin in 5 minutes!

## Prerequisites

- Rust 1.70+ installed ([Install here](https://rustup.rs/))

## Step-by-Step Setup

### 1️⃣ Clone and Build

```bash
git clone https://github.com/yourusername/reuCoin.git
cd reuCoin
cargo build --release
```

### 2️⃣ Start a Node (Terminal 1)

```bash
cargo run --bin node -- --port 9000
```

You should see:
```
Listening on 0.0.0.0:9000
```

### 3️⃣ Start Mining (Terminal 2)

```bash
cargo run --bin miner -- --address 127.0.0.1:9000 --public-key-file ./src/miner/alice.pub.pem
```

### 4️⃣ Generate Wallet Config (Terminal 3)

```bash
cargo run --bin wallet -- generate-config --output wallet_config.toml
```

### 5️⃣ Run the Wallet (Terminal 3)

```bash
cargo run --bin wallet -- --config wallet_config.toml --node 127.0.0.1:9000
```

### 6️⃣ Try Commands

```
> balance
Current balance: 0 satoshis
> exit
```

✅ **Done!** You now have a running blockchain system!

## What Happens Next?

- The **node** maintains the blockchain and accepts connections
- The **miner** continuously mines blocks and earns rewards
- The **wallet** connects to the node to check balance and send transactions

## Common Commands

### Check Available Options

```bash
cargo run --bin node -- --help
cargo run --bin miner -- --help
cargo run --bin wallet -- --help
```

### Run Multiple Nodes

```bash
# Terminal 1: First node (seed)
cargo run --bin node -- --port 9000

# Terminal 2: Second node connecting to first
cargo run --bin node -- --port 9001 127.0.0.1:9000
```

### Run Multiple Miners

```bash
# Different terminals, all connecting to same node
cargo run --bin miner -- --address 127.0.0.1:9000 --public-key-file ./src/miner/alice.pub.pem
```

## Troubleshooting

**Q: Port 9000 already in use?**
```bash
# Use a different port
cargo run --bin node -- --port 9001
```

**Q: "Failed to connect to node"?**
- Make sure the node is running first
- Check the address and port are correct

**Q: Wallet shows balance 0?**
- Mining rewards go to the miner's public key
- You need to get coins from the miner to your wallet

**Q: Want a clean blockchain?**
```bash
rm blockchain.cbor
# Then restart the node
```

## Next Steps

- Read the full [README.md](README.md)
- Explore the source code in `src/`
- Try sending transactions with `send` command
- Run multiple nodes to test network

---

**Need help?** Check the [README.md](README.md) for detailed documentation!

