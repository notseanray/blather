use anyhow::{anyhow, Result};
use blake3::Hasher;
use std::fs::{self, File};
use std::{error::Error, fs::read_dir, io::Read, path::Path};

#[derive(Clone)]
pub struct BinSet {
    timestamp: u64,
    size: u64,
    document_count: usize,
    data_hash: [u8; 32],
}

impl BinSet {
    pub(crate) fn from_folder<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let timestamp = match path.as_ref().to_str().unwrap_or_default().parse() {
            Ok(v) => v,
            _ => return Err(anyhow!("invalid folder name").into()),
        };
        let mut hasher = Hasher::new();
        let mut document_count = 0;
        // reuse this across hash attempts to prevent reallocations and preserve capacity
        let mut buf = Vec::new();
        for file in read_dir(path)? {
            let file = file?;
            if file.file_type()?.is_dir() {
                continue;
            }
            let mut file = File::open(file.path())?;
            file.read_to_end(&mut buf)?;
            hasher.update(buf.as_slice());
            document_count += 1;
            buf.clear();
        }
        Ok(Self {
            timestamp,
            size: hasher.count(),
            document_count,
            data_hash: *hasher.finalize().as_bytes(),
        })
    }
}

pub(crate) struct Storage {
    max_size: f32,
    data: Vec<BinSet>,
}

impl Storage {
    pub(crate) fn new(max_size: f32) -> Result<Self, Box<dyn Error>> {
        let mut data = Vec::with_capacity(8);
        let _ = fs::create_dir("data");
        for folder in read_dir("data")? {
            data.push(BinSet::from_folder(folder?.path())?);
        }
        data.sort_by_key(|x| x.timestamp);
        Ok(Self { max_size, data })
    }

    pub(crate) fn new_download(&mut self) -> Result<()> {
        if self.data.len() < 8 {
            return Ok(());
        }
        // let mut new_ds = V
        Ok(())
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let mut set = Vec::with_capacity(8);
        for folder in read_dir("data")? {
            set.push(BinSet::from_folder(folder?.path())?);
        }
        self.data = set;
        Ok(())
    }

    pub(crate) fn dump(&self) -> &Vec<BinSet> {
        &self.data
    }
}
