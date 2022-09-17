/// Level is a LevelDB wrapper, responsible for opening,
/// closing, and managing databases.
///
/// On creation, a single, internal database will be created
/// as a record of all client-created databases.
pub struct Level {
    db: leveldb::DB,
}

impl Level {
    pub async fn new() -> Result<Self, leveldb::Status> {
        let mut opts = leveldb::Options::default();
        opts.create_if_missing = true;
        opts.error_if_exists = true;

        let db = leveldb::DB::open("internaldb", opts)?;

        Ok(Self { db })
    }
}
