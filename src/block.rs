use crate::block_data::BlockData;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub prev_block_hash: Vec<u8>,
    pub nonce: u64,
    pub data: BlockData,
    pub hash: Vec<u8>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            index: 0,
            timestamp: 0,
            prev_block_hash: vec![0; 32],
            nonce: 0,
            data: BlockData {
                current_value: 0,
                previous_value: 0,
                received_value: 0,
                sent_value: 0,
            },
            hash: rand::random::<[u8; 32]>().to_vec(),
        }
    }

    pub fn next_block(prev_block: &Block, data: BlockData) -> Self {
        let mut block = Block::new();

        block.index = prev_block.index + 1;
        block.timestamp = 0;
        block.prev_block_hash = prev_block.hash.clone();
        block.data = data;
        block.hash = rand::random::<[u8; 32]>().to_vec();

        block
    }
}
