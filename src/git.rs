use git2::Repository;

struct Iteration {
    id: Vec<u8>,
    message: String,
}

pub(crate) struct GitStorage {
    storages: Vec<Iteration>,
}

impl GitStorage {
    pub(crate) fn init() -> Self {
        unimplemented!();
    }
    pub(crate) fn update(&mut self) -> Result<(), ()> {
        let mut repo = Repository::open("./data")
    }
}
