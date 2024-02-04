pub mod tables {
    pub const FILENAMES: redb::TableDefinition<&str, bool> = redb::TableDefinition::new("filenames");
}

pub struct Database {
    pub conn: redb::Database
}

impl Database {
    pub fn new() -> Self {
        Self {
            conn: redb::Database::create("db.cdn_filenames.redb").unwrap()
        }
    }
}