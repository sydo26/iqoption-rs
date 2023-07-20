pub enum IQOptionError {
    /// The credentials are invalid.
    ///
    /// This error is returned when the credentials are invalid.
    ///
    /// Example: The user is not registered or the password is incorrect.
    InvalidCredentials,

    /// The credentials are empty.
    ///
    /// This error is returned when the credentials are not informed.
    EmptyCredentials { message: String },

    /// The session is invalid.
    ///
    /// This error is returned when the session is invalid or expired.
    InvalidSession,

    /// The user is not authorized to perform this action.
    ///
    /// This error is returned when the user is not logged in and tries to
    /// perform an action that requires authorization.
    NotAuthorized,
}

impl std::fmt::Display for IQOptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IQOptionError::InvalidCredentials => {
                write!(f, "The credentials are invalid.")
            }
            IQOptionError::EmptyCredentials { message } => {
                write!(f, "The credentials are empty. {}", message)
            }
            IQOptionError::InvalidSession => {
                write!(f, "The session is invalid.")
            }
            IQOptionError::NotAuthorized => {
                write!(f, "The user is not authorized to perform this action.")
            }
        }
    }
}

impl std::fmt::Debug for IQOptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IQOptionError::InvalidCredentials => {
                write!(f, "IQOptionError::InvalidCredentials")
            }
            IQOptionError::EmptyCredentials { message } => {
                write!(
                    f,
                    "IQOptionError::EmptyCredentials {{ message: {} }}",
                    message
                )
            }
            IQOptionError::InvalidSession => {
                write!(f, "IQOptionError::InvalidSession")
            }
            IQOptionError::NotAuthorized => {
                write!(f, "IQOptionError::NotAuthorized")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iqoption_error_display() {
        assert_eq!(
            format!("{}", IQOptionError::InvalidCredentials),
            "The credentials are invalid."
        );
        assert_eq!(
            format!(
                "{}",
                IQOptionError::EmptyCredentials {
                    message: "test".to_string()
                }
            ),
            "The credentials are empty. test"
        );
        assert_eq!(
            format!("{}", IQOptionError::InvalidSession),
            "The session is invalid."
        );
        assert_eq!(
            format!("{}", IQOptionError::NotAuthorized),
            "The user is not authorized to perform this action."
        );
    }

    #[test]
    fn test_iqoption_error_debug() {
        assert_eq!(
            format!("{:?}", IQOptionError::InvalidCredentials),
            "IQOptionError::InvalidCredentials"
        );
        assert_eq!(
            format!(
                "{:?}",
                IQOptionError::EmptyCredentials {
                    message: "test".to_string()
                }
            ),
            "IQOptionError::EmptyCredentials { message: test }"
        );
        assert_eq!(
            format!("{:?}", IQOptionError::InvalidSession),
            "IQOptionError::InvalidSession"
        );
        assert_eq!(
            format!("{:?}", IQOptionError::NotAuthorized),
            "IQOptionError::NotAuthorized"
        );
    }
}
