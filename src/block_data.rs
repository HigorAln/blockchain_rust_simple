#[derive(Debug)]
pub struct BlockData {
    pub current_value: u64,
    pub previous_value: u64,
    pub received_value: u64,
    pub sent_value: u64,
}
