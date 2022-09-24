#[derive(Debug)]
pub enum FirebaseAuthError {
    TokenExpired,
    UserDisabled,
    UserNotFound,
    ApiKeyInvalid,
    AuthDataMissing,
    EmailExists,
    OperationNotAllowed,
    TooManyAttemptsTryLater,
    EmailNotFound,
    InvalidPassword,
    ParseError(url::ParseError),
    NetworkError(String),
    FirebaseUnexpectedError(String),
    Internal(String),
}

impl std::fmt::Display for FirebaseAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err_msg = match self {
            FirebaseAuthError::TokenExpired =>
                "User's credentials are no longer valid. They must log in again.".to_string(),
            FirebaseAuthError::UserDisabled =>
                "The user account was disabled by an administrator.".to_string(),
            FirebaseAuthError::UserNotFound =>
                "Corresponding user was not found. It is likely the user was deleted.".to_string(),
            FirebaseAuthError::ApiKeyInvalid =>
                "The provided API key is invalid.".to_string(),
            FirebaseAuthError::AuthDataMissing =>
                "Attempting to run protected methods without authenticating. Log in first.".to_string(),
            FirebaseAuthError::EmailExists =>
                "The provided email is already in use".to_string(),
            FirebaseAuthError::OperationNotAllowed =>
                "Password sign-in is disabled for this project".to_string(),
            FirebaseAuthError::TooManyAttemptsTryLater =>
                "Requests from this device were blocked due to unusual activity. Try again later".to_string(),
            FirebaseAuthError::EmailNotFound =>
                "There is no user record corresponding to this email. They might have been deleted".to_string(),
            FirebaseAuthError::InvalidPassword =>
                "The password is invalid or the user does not have a password".to_string(),
            FirebaseAuthError::ParseError(error) =>
                format!("There was an error parsing the URL: {}", error),
            FirebaseAuthError::NetworkError(msg) =>
                format!("Network error occurred: {}", msg),
            FirebaseAuthError::FirebaseUnexpectedError(msg) =>
                format!("Unknown response from Firebase was received: {}", msg),
            FirebaseAuthError::Internal(msg) =>
                format!("Internal error happened: {}", msg),
        };
        write!(f, "{}", err_msg)
    }
}

impl From<url::ParseError> for FirebaseAuthError {
    fn from(error: url::ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl From<serde_json::Error> for FirebaseAuthError {
    fn from(err: serde_json::Error) -> Self {
        Self::FirebaseUnexpectedError(format!("{}", err))
    }
}

impl From<reqwest::Error> for FirebaseAuthError {
    fn from(err: reqwest::Error) -> Self {
        Self::NetworkError(format!("{}", err))
    }
}
