use std::{collections::VecDeque, path::Path, fs::read_dir};
use blake3::Hasher;

pub(crate) struct BinSet {
    timestamp: u64,
    size: u64,
    document_count: usize,
    data_hash: &[u8; 32],
}

impl BinSet {
    pub(crate) fn from_folder(path: AsRef<Path>) -> Result<Self> {
        let mut hasher = blake3::Hasher::new();
        for file in read_dir(path)? {

        }
        unimplemented!();
    }
}

pub(crate) struct Storage {
    max_size: f32,
    data: VecDeque<BinSet>,
}
