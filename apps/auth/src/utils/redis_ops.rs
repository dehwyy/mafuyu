use redis::AsyncCommands;

const EMAIL_CODE_EXPIRE_TIME: u64 = 5 * 60;

pub struct RedisOperations {
    redis_client: redis::Client
}

impl RedisOperations {
    pub fn new(redis_client: redis::Client) -> Self {
       Self {
           redis_client
       }
    }

    pub async fn set_email_code(&self, email: &str, code: &str) -> Result<(), redis::RedisError> {
        let mut redis_conn = self.redis_client.get_async_connection().await?;
        redis_conn.set_ex(email, code, EMAIL_CODE_EXPIRE_TIME).await
    }

    pub async fn get_email_code(&self, email: &str) -> Result<String, redis::RedisError> {
        let mut redis_conn = self.redis_client.get_async_connection().await?;
        redis_conn.get(email).await
    }
}

