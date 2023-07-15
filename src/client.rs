use crate::{Credentials, Enviroment};

/// Client for IQOption API
pub struct IQOptionClient {
    credentials: Credentials,
    enviroment: Enviroment,
}

/// Default value for IQOptionClient
impl Default for IQOptionClient {
    fn default() -> Self {
        let credentials = Credentials {
            identification: None,
            password: None,
        };

        let enviroment = Enviroment::default();

        IQOptionClient {
            credentials,
            enviroment,
        }
    }
}

/// Implementation of methods for IQOptionClient
impl IQOptionClient {
    /// Fill identification field.
    pub fn identification(mut self, identification: &str) -> Self {
        self.credentials.identification = Some(identification.to_string());

        self
    }

    /// Fill password field.
    pub fn password(mut self, password: &str) -> Self {
        self.credentials.password = Some(password.to_string());

        self
    }

    /// Set enviroment for IQOption API.
    /// Specify which environment settings will be used, such as the user-agent.
    /// Default is `Envirment::Simple`.
    pub fn enviroment(mut self, enviroment: Enviroment) -> Self {
        self.enviroment = enviroment;

        self
    }

    /// Connect to IQOption API with credentials.
    pub fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!("Not implemented yet")
    }
}

/// Unit tests for IQOptionClient
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_identification() {
        let client = IQOptionClient::default().identification("test");

        assert_eq!(client.credentials.identification, Some("test".to_string()));
    }

    #[test]
    fn test_password() {
        let client = IQOptionClient::default().password("test");

        assert_eq!(client.credentials.password, Some("test".to_string()));
    }

    #[test]
    fn test_enviroment() {
        assert_eq!(
            IQOptionClient::default()
                .enviroment(Enviroment::Simple)
                .enviroment,
            Enviroment::Simple
        );

        assert_eq!(IQOptionClient::default().enviroment, Enviroment::Simple);
    }

    #[test]
    #[should_panic]
    fn test_connect() {
        let client = IQOptionClient::default();

        assert!(client.connect().is_err());
    }
}
