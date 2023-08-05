use crate::{block_data::BlockData, blockchain::Blockchain};

pub(crate) mod block;
pub mod block_data;
pub(crate) mod blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    for i in 1..1000000000000000 {
        let current_block = blockchain.add_block(BlockData {
            current_value: i * 1000,
            previous_value: 0,
            received_value: 0,
            sent_value: 0,
        });

        println!(
            "Prev. hash: {:?}",
            current_block
                .prev_block_hash
                .iter()
                .map(|hash| format!("{:02x}", hash))
                .collect::<String>()
        );
        println!("Data: {:?}", current_block.data);
        println!(
            "Hash: {:?}",
            current_block
                .hash
                .iter()
                .map(|hash| format!("{:02x}", hash))
                .collect::<String>()
        );
        println!("PoW: {}", blockchain.is_valid());
        println!();
    }

    println!("Hello, world!");
}
