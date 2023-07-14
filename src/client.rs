struct Credentials {
    pub identification: Option<String>,
    pub password: Option<String>,
}

/// Client for IQOption API
pub struct IQOptionClient {
    credentials: Credentials,
}

impl Default for IQOptionClient {
    fn default() -> Self {
        let credentials = Credentials {
            identification: None,
            password: None,
        };

        IQOptionClient { credentials }
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

    /// Connect to IQOption API with credentials.
    pub fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!("Not implemented yet")
    }
}
