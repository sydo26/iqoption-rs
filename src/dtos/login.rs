use crate::{Credentials, Session};

/// Response when login is successful.
///
/// We have more data here, but the other data will not be useful to us here.
pub struct LoginSuccessResponse {
    /// SSID of the session.
    /// This is used to authenticate the user in the future.
    pub ssid: String,
}

/// Data for login request body
pub struct LoginPayload {
    pub identifier: String,
    pub password: String,
}

/// Implementation of `LoginSuccessResponse`
impl LoginSuccessResponse {
    /// Create a new `LoginSuccessResponse`
    pub fn new(ssid: String) -> Self {
        Self { ssid }
    }
}

/// Implementation of `LoginPayload`
impl LoginPayload {
    /// Create a new `LoginPayload`
    pub fn new(identifier: String, password: String) -> Self {
        Self {
            identifier,
            password,
        }
    }
}

/// Convertion from `Credentials` to `LoginPayload`
impl From<Credentials> for LoginPayload {
    fn from(credentials: Credentials) -> Self {
        Self {
            // If the value is None, we set it to an empty string
            identifier: credentials.identification.unwrap_or("".to_string()),
            password: credentials.password.unwrap_or("".to_string()),
        }
    }
}

/// Convertion from `LoginPayload` to `Credentials`
impl From<LoginPayload> for Credentials {
    fn from(payload: LoginPayload) -> Self {
        Self {
            // If the value is empty string, we set it to None
            identification: if payload.identifier.is_empty() {
                None
            } else {
                Some(payload.identifier)
            },
            password: if payload.password.is_empty() {
                None
            } else {
                Some(payload.password)
            },
        }
    }
}

/// Convertion from `Session` to `LoginSuccessResponse`
impl From<Session> for LoginSuccessResponse {
    fn from(session: Session) -> Self {
        Self { ssid: session.ssid }
    }
}

/// Convertion from `LoginSuccessResponse` to `Session`
impl From<LoginSuccessResponse> for Session {
    fn from(response: LoginSuccessResponse) -> Self {
        Self {
            ssid: response.ssid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_payload_to_credentials() {
        let credentials = Credentials::from(LoginPayload {
            identifier: "test".to_string(),
            password: "test".to_string(),
        });

        assert_eq!(credentials.identification, Some("test".to_string()));
        assert_eq!(credentials.password, Some("test".to_string()));
    }

    #[test]
    fn test_login_payload_to_credentials_with_empty_strings() {
        let credentials = Credentials::from(LoginPayload {
            identifier: "".to_string(),
            password: "".to_string(),
        });

        assert_eq!(credentials.identification, None);
        assert_eq!(credentials.password, None);
    }

    #[test]
    fn test_credentials_to_login_payload() {
        let payload = LoginPayload::from(Credentials {
            identification: Some("test".to_string()),
            password: Some("test".to_string()),
        });

        assert_eq!(payload.identifier, "test".to_string());
        assert_eq!(payload.password, "test".to_string());
    }

    #[test]
    fn test_credentials_to_login_payload_with_empty_values() {
        let payload: LoginPayload = LoginPayload::from(Credentials {
            identification: None,
            password: None,
        });

        assert_eq!(payload.identifier, "".to_string());
        assert_eq!(payload.password, "".to_string());
    }

    #[test]
    fn test_login_success_response_to_session() {
        let session = Session::from(LoginSuccessResponse {
            ssid: "test".to_string(),
        });

        assert_eq!(session.ssid, "test".to_string());
    }

    #[test]
    fn test_session_to_login_success_response() {
        let response = LoginSuccessResponse::from(Session {
            ssid: "test".to_string(),
        });

        assert_eq!(response.ssid, "test".to_string());
    }

    #[test]
    fn test_new_instance_of_login_payload() {
        let payload = LoginPayload::new("test".to_string(), "test".to_string());

        assert_eq!(payload.identifier, "test".to_string());
        assert_eq!(payload.password, "test".to_string());
    }

    #[test]
    fn test_new_instance_of_login_response() {
        let response = LoginSuccessResponse::new("test".to_string());

        assert_eq!(response.ssid, "test".to_string());
    }
}
