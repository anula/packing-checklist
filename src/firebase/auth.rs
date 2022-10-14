// TODO: remove dead code allows
#![allow(dead_code)]
#![allow(unused_variables)]

use url::Url;
use serde::{Deserialize, Serialize};

use super::types::*;

pub use super::auth_errors::FirebaseAuthError;

pub type UserId = String;

static IDENTITYTOOLKIT_URL_SUFFIX: &'static str = "identitytoolkit.googleapis.com/v1/";
static SECURETOKEN_URL_SUFFIX: &'static str = "securetoken.googleapis.com/v1/";
static DEFAULT_URL_BASE: &'static str = "https://";


// Implemented most methods from https://firebase.google.com/docs/reference/rest/auth, ignoring
// anything related to third party identity provider.
#[derive(Clone, Debug)]
pub struct FirebaseAuth {
    // wasm version of reqwest's Client does not seem to do any connection pooling, since it should
    // be handled by the browser. Still, this struct will try to use the same (potentially cloned
    // since it is behind an Arc) client wherever possible.
    client: reqwest::Client,
    identitytoolkit_endpoint: String,
    securetoken_endpoint: String,

    api_key: String,
    id_token: Option<String>,
    refresh_token: Option<String>,
}

pub type Result<T> = std::result::Result<T, FirebaseAuthError>;

impl FirebaseAuth {
    pub fn new(api_key: &str) -> Result<Self> {
        FirebaseAuth::new_custom_url_base(api_key, DEFAULT_URL_BASE)
    }

    pub fn new_custom_url_base(api_key: &str, url_base: &str) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            identitytoolkit_endpoint: format!("{}{}", url_base, IDENTITYTOOLKIT_URL_SUFFIX),
            securetoken_endpoint: format!("{}{}", url_base, SECURETOKEN_URL_SUFFIX),

            api_key: api_key.to_owned(),
            id_token: None,
            refresh_token: None,
        })
    }

    // Exchange a refresh token for an ID token
    // https://firebase.google.com/docs/reference/rest/auth#section-refresh-token
    pub async fn refresh_id_token(&mut self) -> Result<()> {
        let url = Url::parse_with_params(
            &format!("{}{}", self.securetoken_endpoint, "token"),
            &[("key", &self.api_key)]
        )?;
        let refresh_token = self.refresh_token.as_ref().ok_or(FirebaseAuthError::AuthDataMissing)?;
        let request = self.client.post(url.as_str())
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &refresh_token),
            ]);
        let response = make_request::<RefreshIdTokenResponse>(request, |err| {
            match err.message {
                FirebaseErrorMessage::TokenExpired => FirebaseAuthError::TokenExpired,
                FirebaseErrorMessage::UserDisabled => FirebaseAuthError::UserDisabled,
                FirebaseErrorMessage::UserNotFound => FirebaseAuthError::UserNotFound,
                FirebaseErrorMessage::ApiKeyInvalid => FirebaseAuthError::ApiKeyInvalid,
                our_err @ (FirebaseErrorMessage::InvalidRefreshToken |
                    FirebaseErrorMessage::InvalidGrantType |
                    FirebaseErrorMessage::MissingRefreshToken) =>
                    FirebaseAuthError::Internal(format!("{}", our_err)),
                unexpected_err =>
                    FirebaseAuthError::FirebaseUnexpectedError(format!("{}", unexpected_err)),
            }
        }).await?;
        self.refresh_token = Some(response.refresh_token);
        self.id_token = Some(response.id_token);
        Ok(())
    }

    // Sign up with email / password
    // https://firebase.google.com/docs/reference/rest/auth#section-create-email-password
    pub async fn sign_up(&mut self, email: &str, password: &str) -> Result<()> {
        let url = Url::parse_with_params(
            &format!("{}{}", self.identitytoolkit_endpoint, "accounts:signUp"),
            &[("key", &self.api_key)]
        )?;
        let request = self.client.post(url.as_str())
            .json(&SignUpRequest {
                email: email.to_string(),
                password: password.to_string(),
                return_secure_token: true,
            });
        let response = make_request::<SignUpResponse>(request, |err| {
            match err.message {
                FirebaseErrorMessage::EmailExists => FirebaseAuthError::EmailExists,
                FirebaseErrorMessage::OperationNotAllowed => FirebaseAuthError::OperationNotAllowed,
                FirebaseErrorMessage::TooManyAttemptsTryLater => FirebaseAuthError::TooManyAttemptsTryLater,
                unexpected_err =>
                    FirebaseAuthError::FirebaseUnexpectedError(format!("{}", unexpected_err)),
            }
        }).await?;
        self.refresh_token = Some(response.refresh_token);
        self.id_token = Some(response.id_token);
        Ok(())
    }

    // Sign in with email / password
    // https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password
    pub async fn sign_in(&mut self, email: &str, password: &str) -> Result<UserId> {
        let url = Url::parse_with_params(
            &format!("{}{}", self.identitytoolkit_endpoint, "accounts:signInWithPassword"),
            &[("key", &self.api_key)]
        )?;
        let request = self.client.post(url.as_str())
            .json(&SignInRequest {
                email: email.to_string(),
                password: password.to_string(),
                return_secure_token: true,
            });
        let response = make_request::<SignInResponse>(request, |err| {
            match err.message {
                FirebaseErrorMessage::EmailNotFound => FirebaseAuthError::EmailNotFound,
                FirebaseErrorMessage::InvalidPassword => FirebaseAuthError::InvalidPassword,
                FirebaseErrorMessage::UserDisabled => FirebaseAuthError::UserDisabled,
                unexpected_err =>
                    FirebaseAuthError::FirebaseUnexpectedError(format!("{}", unexpected_err)),
            }
        }).await?;
        self.refresh_token = Some(response.refresh_token);
        self.id_token = Some(response.id_token);
        Ok(response.local_id)
    }

    // Change email
    // https://firebase.google.com/docs/reference/rest/auth#section-change-email
    pub async fn change_email(&mut self, new_email: &str) -> Result<()> {
        let url = Url::parse_with_params(
            &format!("{}{}", self.identitytoolkit_endpoint, "accounts:update"),
            &[("key", &self.api_key)]
        )?;
        let id_token = self.id_token.as_ref().ok_or(FirebaseAuthError::AuthDataMissing)?;
        let request = self.client.post(url.as_str())
            .json(&ChangeEmailRequest{
                id_token: id_token.to_string(),
                email: new_email.to_string(),
                return_secure_token: true,
            });
        let response = make_request::<ChangeEmailResponse>(request, |err| {
            match err.message {
                FirebaseErrorMessage::EmailExists => FirebaseAuthError::EmailExists,
                FirebaseErrorMessage::InvalidIdToken => FirebaseAuthError::InvalidIdToken,
                unexpected_err =>
                    FirebaseAuthError::FirebaseUnexpectedError(format!("{}", unexpected_err)),
            }
        }).await?;
        self.id_token = Some(response.id_token);
        self.refresh_token = Some(response.refresh_token);
        Ok(())
    }

    // Change password
    // https://firebase.google.com/docs/reference/rest/auth#section-change-password
    pub async fn change_password(&mut self, new_password: &str) -> Result<()> {
        let url = Url::parse_with_params(
            &format!("{}{}", self.identitytoolkit_endpoint, "accounts:update"),
            &[("key", &self.api_key)]
        )?;
        let id_token = self.id_token.as_ref().ok_or(FirebaseAuthError::AuthDataMissing)?;
        let request = self.client.post(url.as_str())
            .json(&ChangePasswordRequest{
                id_token: id_token.to_string(),
                password: new_password.to_string(),
                return_secure_token: true,
            });
        let response = make_request::<ChangePasswordResponse>(request, |err| {
            match err.message {
                FirebaseErrorMessage::InvalidIdToken => FirebaseAuthError::InvalidIdToken,
                FirebaseErrorMessage::WeakPassword => FirebaseAuthError::WeakPassword,
                unexpected_err =>
                    FirebaseAuthError::FirebaseUnexpectedError(format!("{}", unexpected_err)),
            }
        }).await?;
        self.id_token = Some(response.id_token);
        self.refresh_token = Some(response.refresh_token);
        Ok(())
    }

    // Partial "Update profile"
    // https://firebase.google.com/docs/reference/rest/auth#section-update-profile
    // Note: we implement only display_name here! This is not all that Firebase supports.
    pub async fn change_display_name(&mut self, new_display_name: &str) -> Result<()> {
        let url = Url::parse_with_params(
            &format!("{}{}", self.identitytoolkit_endpoint, "accounts:update"),
            &[("key", &self.api_key)]
        )?;
        let id_token = self.id_token.as_ref().ok_or(FirebaseAuthError::AuthDataMissing)?;
        let request = self.client.post(url.as_str())
            .json(&UpdateProfileRequest{
                id_token: id_token.to_string(),
                display_name: new_display_name.to_string(),
                return_secure_token: true,
            });
        let response = make_request::<UpdateProfileResponse>(request, |err| {
            match err.message {
                FirebaseErrorMessage::InvalidIdToken => FirebaseAuthError::InvalidIdToken,
                unexpected_err =>
                    FirebaseAuthError::FirebaseUnexpectedError(format!("{}", unexpected_err)),
            }
        }).await?;
        if let Some(new_id_token) = response.id_token {
            self.id_token = Some(new_id_token);
        }
        if let Some(new_refresh_token) = response.refresh_token {
            self.refresh_token = Some(new_refresh_token);
        }
        Ok(())
    }

    // Partial "Get user data"
    // https://firebase.google.com/docs/reference/rest/auth#section-get-account-info
    // Note: we implement only display_name here! This is not all that Firebase supports.
    pub async fn get_display_name(&mut self) -> Result<Option<String>> {
        let url = Url::parse_with_params(
            &format!("{}{}", self.identitytoolkit_endpoint, "accounts:lookup"),
            &[("key", &self.api_key)]
        )?;
        let id_token = self.id_token.as_ref().ok_or(FirebaseAuthError::AuthDataMissing)?;
        let request = self.client.post(url.as_str())
            .json(&GetUserDataRequest{
                id_token: id_token.to_string(),
            });
        let response = make_request::<GetUserDataResponse>(request, |err| {
            match err.message {
                FirebaseErrorMessage::InvalidIdToken => FirebaseAuthError::InvalidIdToken,
                FirebaseErrorMessage::UserNotFound => FirebaseAuthError::UserNotFound,
                unexpected_err =>
                    FirebaseAuthError::FirebaseUnexpectedError(format!("{}", unexpected_err)),
            }
        }).await?;
        if response.users.len() < 1 {
            return Err(
                FirebaseAuthError::FirebaseUnexpectedError(format!("No user data was returned.")));
        }
        Ok(response.users[0].display_name.clone())
    }
}

async fn make_request<Resp>(
    request: reqwest::RequestBuilder, map_firebase_err: fn(FirebaseErrorDetails) -> FirebaseAuthError)
    -> Result<Resp> where for<'de> Resp: Deserialize<'de>
{
    let response = request.send().await?;
    if response.status().is_success() {
        let resp_text = response.text().await?;
        Ok(serde_json::from_str::<Resp>(&resp_text)?)
    } else {
        let resp_text = response.text().await?;
        let firebase_resp = serde_json::from_str::<FirebaseErrorResponse>(&resp_text)?;
        Err(map_firebase_err(firebase_resp.error))
    }
}

// === Request/Response types ===
// Each method of Firebase Auth REST API might return different data.
// Here we define response types for each of the methods we use. Note that they often do not
// include all the fields that can be returned, only the ones relevant for the application - serde
// deals with that.

#[derive(Debug, PartialEq, Deserialize)]
struct RefreshIdTokenResponse {
    refresh_token: String,
    id_token: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignUpRequest {
    email: String,
    password: String,
    return_secure_token: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignUpResponse {
    id_token: String,
    refresh_token: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignInRequest {
    email: String,
    password: String,
    return_secure_token: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignInResponse {
    id_token: String,
    refresh_token: String,
    local_id: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChangeEmailRequest {
    id_token: String,
    email: String,
    return_secure_token: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangeEmailResponse {
    email: String,
    id_token: String,
    refresh_token: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChangePasswordRequest {
    id_token: String,
    password: String,
    return_secure_token: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangePasswordResponse {
    email: String,
    id_token: String,
    refresh_token: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProfileRequest {
    id_token: String,
    display_name: String,
    return_secure_token: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProfileResponse {
    id_token: Option<String>,
    refresh_token: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetUserDataRequest {
    id_token: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserDetailsResponse {
    local_id: String,
    email: String,
    display_name: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetUserDataResponse {
    users: Vec<UserDetailsResponse>,
}

// === Request/Response types ===

#[cfg(test)]
mod firebase_auth_tests {
    use super::*;
    use k9::assert_ok;

    #[test]
    fn test_default_addresses() {
        assert_ok!(FirebaseAuth::new("api_key"));
    }

}

#[cfg(test)]
mod firebase_auth_local_emulator_tests {
    // These tests work only with a local auth emulator running, on port 9099.
    use super::*;
    use crate::config::parse_from_embedded_file;
    use k9::assert_ok;
    use std::sync::Once;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::variant::*;

    static GLOBAL_SETUP: Once = Once::new();

    fn global_setup() {
        GLOBAL_SETUP.call_once(|| {
            clean_up_local_emulator();
        });
    }

    fn get_host_from_config() -> String {
        let config = parse_from_embedded_file().unwrap();
        config.auth_host
    }

    fn clean_up_local_emulator() {
        let project_id = "packing-checklist-3879";
        let host = get_host_from_config();
        assert_ok!(tokio_test::block_on(
            reqwest::Client::new().delete(
                &format!("{}emulator/v1/projects/{}/accounts", host, project_id))
            .send()
        ));
    }

    #[test]
    fn test_new_user_flow() {
        global_setup();
        let mut auth = FirebaseAuth::new_custom_url_base("api_key", &get_host_from_config()).unwrap();
        assert_ok!(tokio_test::block_on(auth.sign_up("user@example.com", "password")));
        assert_ok!(tokio_test::block_on(auth.sign_in("user@example.com", "password")));
        assert_ok!(tokio_test::block_on(auth.refresh_id_token()));
    }

    #[test]
    fn test_display_name_change() {
        global_setup();
        let mut auth = FirebaseAuth::new_custom_url_base("api_key", &get_host_from_config()).unwrap();

        assert_ok!(tokio_test::block_on(auth.sign_up("user1@example.com", "password")));
        assert_ok!(tokio_test::block_on(auth.sign_in("user1@example.com", "password")));

        assert_eq!(tokio_test::block_on(auth.get_display_name()).unwrap(), None);
        assert_ok!(tokio_test::block_on(auth.change_display_name("new_name")));
        assert_that!(
            &tokio_test::block_on(auth.get_display_name()).unwrap(),
            maybe_some(eq("new_name".to_string()))
        );
    }

    #[test]
    fn test_password_change() {
        global_setup();
        let mut auth = FirebaseAuth::new_custom_url_base("api_key", &get_host_from_config()).unwrap();

        assert_ok!(tokio_test::block_on(auth.sign_up("user2@example.com", "password")));
        assert_ok!(tokio_test::block_on(auth.sign_in("user2@example.com", "password")));
        assert_that!(
            &tokio_test::block_on(auth.sign_in("user2@example.com", "wrong_password")),
            maybe_err(eq(FirebaseAuthError::InvalidPassword))
        );
        assert_ok!(tokio_test::block_on(auth.change_password("new_password")));
        assert_that!(
            &tokio_test::block_on(auth.sign_in("user2@example.com", "password")),
            maybe_err(eq(FirebaseAuthError::InvalidPassword))
        );
        assert_ok!(tokio_test::block_on(auth.sign_in("user2@example.com", "new_password")));
    }

    #[test]
    fn test_email_change() {
        global_setup();
        let mut auth = FirebaseAuth::new_custom_url_base("api_key", &get_host_from_config()).unwrap();

        assert_ok!(tokio_test::block_on(auth.sign_up("user3@example.com", "password3")));
        assert_ok!(tokio_test::block_on(auth.sign_in("user3@example.com", "password3")));

        assert_ok!(tokio_test::block_on(auth.change_email("new_email@new_email.com")));

        assert_that!(
            &tokio_test::block_on(auth.sign_in("user3@example.com", "password3")),
            maybe_err(eq(FirebaseAuthError::EmailNotFound))
        );
        assert_ok!(tokio_test::block_on(auth.sign_in("new_email@new_email.com", "password3")));
    }
}
