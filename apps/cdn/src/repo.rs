use redb::ReadableTable;
use makoto_logger::info;

pub struct Repo {
    db: crate::db::Database
}

impl Repo {
    pub fn new(db: crate::db::Database) -> Self {
        Self {
           db
        }
    }

    pub async fn has_value_by_key(&self, key: &str) -> Result<bool, redb::Error> {
        let read_tx = self.db.conn.begin_read()?;
        let table = read_tx.open_table(crate::db::tables::FILENAMES);

        match table {
            Ok(table) => {
                let found_record = table.get(key)?;
                Ok(found_record.is_some())
            },
            Err(_) => {
                Ok(false)
            }
        }

    }

    pub async fn reserve_key(&self, key: &str) -> Result<(), redb::Error> {
        let write_txn = self.db.conn.begin_write()?;
        {
            let mut table = write_txn.open_table(crate::db::tables::FILENAMES)?;
            table.insert(key, true)?;
        }

        write_txn.commit()?;

        Ok(())
    }
}