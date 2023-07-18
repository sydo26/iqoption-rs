use crate::{Credentials, Enviroment, IQOptionClient, Session};

/// Builder for IQOption Client
pub struct IQOptionBuilder {
    credentials: Credentials,
    enviroment: Enviroment,
}

/// Default value for IQOptionClient
impl Default for IQOptionBuilder {
    fn default() -> Self {
        let credentials = Credentials {
            identification: None,
            password: None,
        };

        let enviroment = Enviroment::default();

        IQOptionBuilder {
            credentials,
            enviroment,
        }
    }
}

impl IQOptionBuilder {
    /// Fill identification field.
    #[inline]
    pub fn identification(mut self, identification: &str) -> Self {
        self.credentials.identification = Some(identification.to_string());

        self
    }

    /// Fill password field.
    #[inline]
    pub fn password(mut self, password: &str) -> Self {
        self.credentials.password = Some(password.to_string());

        self
    }

    /// Set enviroment for IQOption API.
    /// Specify which environment settings will be used, such as the user-agent.
    /// Default is `Enviroment::Simple`.
    #[inline]
    pub fn enviroment(mut self, enviroment: Enviroment) -> Self {
        self.enviroment = enviroment;

        self
    }

    /// Build IQOption Client.
    #[inline]
    pub async fn build(self) -> Result<IQOptionClient, Box<dyn std::error::Error>> {
        if self.credentials.identification.is_none() {
            panic!("Identification is not specified");
        } else if self.credentials.password.is_none() {
            panic!("Password is not specified");
        }

        // SignIn to IQOption API
        // ...

        Ok(IQOptionClient::new(
            Session {
                // FIXME: specify ssid
                ssid: "0101".to_string(),
            },
            self.enviroment,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identification() {
        let builder = IQOptionBuilder::default().identification("test");

        assert_eq!(builder.credentials.identification, Some("test".to_string()));
    }

    #[test]
    fn test_password() {
        let builder = IQOptionBuilder::default().password("test");

        assert_eq!(builder.credentials.password, Some("test".to_string()));
    }

    #[test]
    fn test_enviroment() {
        assert_eq!(
            IQOptionBuilder::default()
                .enviroment(Enviroment::Simple)
                .enviroment,
            Enviroment::Simple
        );

        assert_eq!(IQOptionBuilder::default().enviroment, Enviroment::Simple);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_connect_without_all_credentials() {
        IQOptionBuilder::default().build().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Identification is not specified")]
    async fn test_connect_without_identification() {
        IQOptionBuilder::default()
            .password("test")
            .build()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Password is not specified")]
    async fn test_connect_without_password() {
        IQOptionBuilder::default()
            .identification("test")
            .build()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_success_connect() {
        let result = IQOptionBuilder::default()
            .identification("test")
            .password("test")
            .build()
            .await;

        assert!(result.is_ok());
    }
}
