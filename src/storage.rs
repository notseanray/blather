use std::{collections::VecDeque, path::Path, fs::read_dir, error::Error, io::Read};
use std::fs::{File, self};
use blake3::Hasher;
use anyhow::{Result, anyhow};

pub(crate) struct BinSet {
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
        for file in read_dir(path).unwrap() {
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
//SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()

pub(crate) struct Storage {
    max_size: f32,
    data: VecDeque<BinSet>,
}

impl Storage {
    pub(crate) fn new(max_size: f32) -> Result<Self, Box<dyn Error>> {
        let mut dataset = Vec::with_capacity(8);
        let _ = fs::create_dir("data");
        for folder in read_dir("data")? {
            dataset.push(BinSet::from_folder(folder?.path())?);
        }
        dataset.sort_by_key(|x| x.timestamp);
        // let data = VecDeque::with_capacity(dataset.len());
        // take(&mut dataset);
        let data = VecDeque::from_iter(dataset);
        Ok(Self { max_size, data })
    }
    pub(crate) fn update(&mut self) -> Result<()> {
        if self.data.len() < 8 {
            return Ok(());
        }
        Ok(())
    }
}
