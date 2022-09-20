use std::option::{Option};

// Empty user_info means user is unauthenticated.
// Non-empty user_info has the authentication data for the user from the last communication
// with the backend.
#[derive(Clone, PartialEq)]
pub struct UserAuthStatus {
    pub user_info: Option<UserInfo>,
}

impl UserAuthStatus {
    pub fn new_empty() -> Self {
        Self {
            user_info: None,
        }
    }

    //pub fn is_authenticated(&self) -> bool { self.user_info != Option::None }
}

#[derive(Clone, PartialEq)]
pub struct UserInfo {
    pub display_name: String,
    pub email: String,
    pub id_token: String,
    pub refresh_token: String,
}
