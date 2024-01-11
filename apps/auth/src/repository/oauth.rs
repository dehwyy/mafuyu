use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct Oauth {
  db: DatabaseConnection
}

impl Oauth {
  pub fn new(db: DatabaseConnection) -> Self {
    Self {
      db
    }
  }

  pub async fn create_empty_record(&self, user_id: Uuid) -> Result<(), String> {
    todo!()
  }
}
