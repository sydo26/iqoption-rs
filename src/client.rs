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
    pub async fn connect(self) -> Result<Self, Box<dyn std::error::Error>> {
        if self.credentials.identification.is_none() {
            panic!("Identification is not specified");
        } else if self.credentials.password.is_none() {
            panic!("Password is not specified");
        }

        // TODO: Implement connection to IQOption API
        Ok(self)
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

    #[tokio::test]
    #[should_panic]
    async fn test_connect_without_all_credentials() {
        let client = IQOptionClient::default();

        client.connect().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Identification is not specified")]
    async fn test_connect_without_identification() {
        let client = IQOptionClient::default().password("test");

        client.connect().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Password is not specified")]
    async fn test_connect_without_password() {
        let client = IQOptionClient::default().identification("test");

        client.connect().await.unwrap();
    }

    #[tokio::test]
    async fn test_success_connect() {
        let client = IQOptionClient::default()
            .identification("test")
            .password("test");

        let result = client.connect().await;

        assert!(result.is_ok());
    }
}
