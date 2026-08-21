mod cli;
mod error;
mod transaction;

use clap::Parser;

use cli::{parse_input, parse_output};
use transaction::{bytes_to_hex, Transaction, TxInput, TxOutput};

/// Construct and serialize a Bitcoin transaction from command-line arguments.
#[derive(Parser, Debug)]
#[command(name = "serialize-trx", about, long_about = None)]
struct Args {
    /// Transaction version (nVersion).
    #[arg(long = "tx-version", default_value_t = 2)]
    tx_version: i32,

    /// Serialize as a SegWit transaction (adds the marker/flag and witness data).
    #[arg(long)]
    segwit: bool,

    /// Locktime for the transaction.
    #[arg(long, default_value_t = 0)]
    locktime: u32,

    /// A transaction input: "txid=<hex>,vout=<u32>[,sequence=<u32>][,script_sig=<hex>][,witness=<hex>|<hex>|...]"
    /// May be repeated to add multiple inputs.
    #[arg(long = "input", required = true, value_parser = parse_input)]
    inputs: Vec<TxInput>,

    /// A transaction output: "value=<u64>,script_pubkey=<hex>"
    /// May be repeated to add multiple outputs.
    #[arg(long = "output", required = true, value_parser = parse_output)]
    outputs: Vec<TxOutput>,
}

fn main() {
    let args = Args::parse();

    if !args.segwit && args.inputs.iter().any(|input| !input.witness.is_empty()) {
        eprintln!(
            "warning: witness data was provided but --segwit was not set; it will not appear in the serialized output"
        );
    }

    let transaction = Transaction {
        version: args.tx_version,
        segwit: args.segwit,
        inputs: args.inputs,
        outputs: args.outputs,
        locktime: args.locktime,
    };

    let serialized = transaction.serialize();

    println!("Serialized transaction (hex):");
    println!("{}", bytes_to_hex(&serialized));
    println!("Transaction size: {} bytes", serialized.len());
}
