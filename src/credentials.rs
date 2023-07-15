use std::fmt::{Debug, Formatter, Result};

/// Credentials for IQOption API
pub struct Credentials {
    pub identification: Option<String>,
    pub password: Option<String>,
}

/// Default value for Credentials
impl Default for Credentials {
    fn default() -> Self {
        const CREDENTIALS: Credentials = Credentials {
            identification: None,
            password: None,
        };

        CREDENTIALS
    }
}

/// Debug implementation for Credentials
impl Debug for Credentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Credentials")
            .field("identification", &self.identification)
            .field("password", &self.password)
            .finish()
    }
}

/// Implementation of methods for Credentials
impl PartialEq for Credentials {
    fn eq(&self, other: &Self) -> bool {
        self.identification == other.identification && self.password == other.password
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default() {
        let credentials = Credentials::default();

        assert_eq!(credentials.identification, None);
        assert_eq!(credentials.password, None);
    }

    #[test]
    fn test_debug() {
        let credentials = Credentials::default();

        assert_eq!(
            format!("{:?}", credentials),
            "Credentials { identification: None, password: None }"
        );
    }

    #[test]
    fn test_eq() {
        assert_eq!(Credentials::default(), Credentials::default());
    }
}
