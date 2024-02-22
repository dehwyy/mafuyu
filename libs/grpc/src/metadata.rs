pub const METADATA_USER_ID_KEY: &str = "x-user-id";
pub const METADATA_USER_ROLE_KEY: &str = "x-user-role";


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