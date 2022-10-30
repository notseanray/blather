use anyhow::Result;
use actix::Message;
use git2::{Commit, Repository};

#[derive(Clone)]
pub struct CommitPoint {
    id: Vec<u8>,
    message: String,
}

impl From<Commit<'_>> for CommitPoint {
    fn from(g: Commit<'_>) -> Self {
        Self {
            id: g.id().as_bytes().to_vec(),
            message: g.message().unwrap().to_string(),
        }
    }
}

impl Message for CommitPoint {
    type Result = Vec<CommitPoint>;
}

#[derive(Default)]
pub(crate) struct GitStorage {
    storages: Vec<CommitPoint>,
}

impl GitStorage {
    pub(crate) fn init() -> Result<Self> {
        let mut storage = Self::default();
        storage.fetch_commits()?;
        Ok(storage)
    }
    pub fn get_commits(&self) -> &Vec<CommitPoint> {
        &self.storages
    }
    fn fetch_commits(&mut self) -> Result<()> {
        let repo = Repository::open("./data")?;
        for commit in repo.revwalk().unwrap() {
            let c = repo.find_commit(commit.unwrap()).unwrap();
            self.storages.push(c.into());
        }
        Ok(())
    }
}
