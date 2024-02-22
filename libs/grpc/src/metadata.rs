pub const METADATA_USER_ID_KEY: &str = "x-user-id";
pub const METADATA_USER_ROLE_KEY: &str = "x-user-role";

pub struct Metadata;

impl Metadata {
    pub fn get_user_role_and_id_from_metadata(metadata: &tonic::metadata::MetadataMap) -> Option<(UserRole, String)> {
        let user_id = metadata.get(METADATA_USER_ID_KEY)
            .map(|v| v.to_str().map(|s| s.to_string()));
        let user_role = metadata.get(METADATA_USER_ROLE_KEY)
            .map(|v| v.to_str().map(|s| s.to_string()));

        match (user_id, user_role) {
            (Some(Ok(user_id)), Some(Ok(user_role))) => Some((UserRole::from(user_role), user_id)),
            _ => None
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum UserRole {
    Unauthorized = 1,
    User = 2,
    Admin = 3,
    CEO = 4
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserRole::Unauthorized => "unauthorized",
            UserRole::User => "user",
            UserRole::Admin => "admin",
            UserRole::CEO => "ceo"
        };

        write!(f, "{}", s)
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "unauthorized" => UserRole::Unauthorized,
            "user" => UserRole::User,
            "admin" => UserRole::Admin,
            "ceo" => UserRole::CEO,
            _ => UserRole::Unauthorized
        }
    }
}

impl From<i16> for UserRole {
    fn from(i: i16) -> Self {
        match i {
            1 => UserRole::Unauthorized,
            2 => UserRole::User,
            3 => UserRole::Admin,
            4 => UserRole::CEO,
            _ => UserRole::Unauthorized
        }
    }
}

impl Into<i32> for UserRole {
    fn into(self) -> i32 {
        match self {
            UserRole::Unauthorized => 1,
            UserRole::User => 2,
            UserRole::Admin => 3,
            UserRole::CEO => 4
        }
    }
}