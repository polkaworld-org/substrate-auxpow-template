# Substrate AuxPow Template 

A new SRML-based Substrate node with AuxPow consensus.

This project implements the same merged mining specification as namecoin. Normally, the miner who could mining namecoin could easily mining this node.

The node is mainly implemented through the following two interfaces:

- createauxblock
- submitauxblock

Bitcoin miners need to do the following:

1. Get the auxblock hash and chain id through `createauxblock` interface.
2. Write the auxblock hash into the coinbase of bitcoin. if there are multiple auxiliary chains, write the merkle root which generated with their auxblock hashes instead. Refer to the merged mining specification.
3. Check if the bitcoin block header which contains the auxblock hash meets this node difficulty.
4. Send the proof of work (auxpow) to this node which meets this node's difficulty through `submitauxblock` interface.

This node does not implement a real-time notification of the task updates by ZMQ, so `createauxblock` needs to be periodically rotated from the mine pool, such as every 10 seconds.

Merged mining specification： https://en.bitcoin.it/wiki/Merged_mining_specification

## Todo

1. Support generic algorithm on parent chain, to adapts to different algorithms, and can easily achieve merged mining with dogcoin and litecoin.
2. Solving the variable length int problem of bitcoin structure deserialization
3. Accept miner address, do not use a fixed address
4. Blockchain reorganization mechanism (seems is already supported by substrate)
5. Implement the full JSON-RPC for mining pool.

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install required tools:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build
```

## Run

### Single node development chain

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Multi-node local testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.
