// Types used for communication with Firebase. Not intended to be used outside of this folder.
use serde::{Deserialize};
use serde::de::IntoDeserializer;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(remote = "FirebaseErrorMessage")]
pub enum FirebaseErrorMessage {
    InvalidRefreshToken,
    CredentialTooOldLoginAgain,
    TokenExpired,
    InvalidIdToken,
    WeakPassword,
    UserDisabled,
    UserNotFound,
    InvalidGrantType,
    MissingRefreshToken,
    InvalidArgument,
    ApiKeyInvalid,
    EmailExists,
    OperationNotAllowed,
    TooManyAttemptsTryLater,
    EmailNotFound,
    InvalidPassword,
    InvalidEmail,
    InvalidJson,
    ExpiredOobCode,
    InvalidOobCode,
    Unparsable(String),
}

impl<'de> Deserialize<'de> for FirebaseErrorMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: serde::Deserializer<'de>
    {
        let str_value = String::deserialize(deserializer)?;
        let try_default: Result<Self, D::Error> = Self::deserialize(str_value.clone().into_deserializer());
        if let Ok(result) = try_default {
            return Ok(result);
        }
        Ok(match str_value.as_str() {
            "API key not valid. Please pass a valid API key." => Self::ApiKeyInvalid,
            val => Self::Unparsable(val.to_string()),
        })
    }
}

impl std::fmt::Display for FirebaseErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FirebaseErrorDetails {
    pub code: i32,
    pub message: FirebaseErrorMessage,
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct FirebaseErrorResponse {
    pub error: FirebaseErrorDetails,
}

#[cfg(test)]
mod firebase_types_tests {
    use super::*;

    #[test]
    fn deserialize_firebase_message() {
        assert_eq!(
            serde_json::from_str::<FirebaseErrorMessage>(r#""API key not valid. Please pass a valid API key.""#).unwrap(),
            FirebaseErrorMessage::ApiKeyInvalid
        );
        assert_eq!(
            serde_json::from_str::<FirebaseErrorMessage>(r#""API_KEY_INVALID""#).unwrap(),
            FirebaseErrorMessage::ApiKeyInvalid
        );
        assert_eq!(
            serde_json::from_str::<FirebaseErrorMessage>(r#""TOKEN_EXPIRED""#).unwrap(),
            FirebaseErrorMessage::TokenExpired
        );
        assert_eq!(
            serde_json::from_str::<FirebaseErrorMessage>(r#""INVALID_REFRESH_TOKEN""#).unwrap(),
            FirebaseErrorMessage::InvalidRefreshToken
        );
        assert_eq!(
            serde_json::from_str::<FirebaseErrorMessage>(r#""Unexpected string.""#).unwrap(),
            FirebaseErrorMessage::Unparsable("Unexpected string.".to_string())
        );
    }

    #[test]
    fn parses_normal_response() {
        assert_eq!(
            serde_json::from_str::<FirebaseErrorResponse>(r#"
                    {
                        "error": {
                            "code": 400,
                            "message": "USER_DISABLED"
                        }
                    }"#).unwrap(),
            FirebaseErrorResponse {
                error: FirebaseErrorDetails {
                    code: 400,
                    message: FirebaseErrorMessage::UserDisabled,
                }
            }
        );
    }

    #[test]
    fn parses_response_with_additional_fields() {
        assert_eq!(
            serde_json::from_str::<FirebaseErrorResponse>(r#"
                    {
                        "error": {
                            "code": 400,
                            "message": "API key not valid. Please pass a valid API key.",
                            "status": "INVALID_ARGUMENT",
                            "details": [
                              {
                                "@type": "type.googleapis.com/google.rpc.ErrorInfo",
                                "reason": "API_KEY_INVALID",
                                "domain": "googleapis.com",
                                "metadata": {
                                  "service": "securetoken.googleapis.com"
                                }
                              }
                            ]
                        }
                    }"#).unwrap(),
            FirebaseErrorResponse {
                error: FirebaseErrorDetails {
                    code: 400,
                    message: FirebaseErrorMessage::ApiKeyInvalid,
                }
            }
        );
    }
}

