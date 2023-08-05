use crate::{block::Block, block_data::BlockData};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::new()],
        }
    }

    pub fn add_block(&mut self, data: BlockData) -> &Block {
        let prev_block = self.blocks.last().unwrap();
        let next_block = data;
        self.blocks.push(Block::next_block(prev_block, next_block));

        self.blocks.last().unwrap().clone()
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];

            if current_block.hash != current_block.hash {
                return false;
            }

            if current_block.prev_block_hash != prev_block.hash {
                return false;
            }
        }

        true
    }
}
