use git2::Repository;
use anyhow::Result;

struct CommitPoint {
    id: Vec<u8>,
    message: String,
}

impl Into<CommitPoint> for git2::Commit<'_> {
    fn into(self) -> CommitPoint {
        CommitPoint { id: self.id().as_bytes().to_vec(), message: self.message().unwrap().to_string() }
    }
}

#[derive(Default)]
pub(crate) struct GitStorage {
    storages: Vec<CommitPoint>,
}

impl GitStorage {
    pub(crate) fn init() -> Result<Self, ()> {
        let mut storage = Self::default();
        storage.fetch_commits()?;
        Ok(storage)
    }
    fn fetch_commits(&mut self) -> Result<(), ()> {
        let repo = Repository::open("./data").unwrap();
        for commit in repo.revwalk().unwrap() {
            let c = repo.find_commit(commit.unwrap()).unwrap();
            self.storages.push(c.into());
        }
        Ok(())
    }
}
