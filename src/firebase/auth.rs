// TODO: remove dead code allows
#![allow(dead_code)]
#![allow(unused_variables)]

use url::Url;
use serde::{Deserialize, Serialize};

use super::types::*;

pub use super::auth_errors::FirebaseAuthError;

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
    pub async fn sign_in(&mut self, email: &str, password: &str) -> Result<()> {
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
        Ok(())
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

    // TODO: write unit tests that don't require local emulator to be running
    //#[test]
    fn test_with_local_emulator() -> Result<()> {
        let mut auth = FirebaseAuth::new_custom_url_base("api_key", "http://localhost:9099/").unwrap();
        assert_ok!(tokio_test::block_on(auth.sign_up("user@example.com", "password")));
        assert_ok!(tokio_test::block_on(auth.sign_in("user@example.com", "password")));
        assert_ok!(tokio_test::block_on(auth.refresh_id_token()));
        Ok(())
    }
}
