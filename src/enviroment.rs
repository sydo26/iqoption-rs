use std::fmt::{self, Debug, Formatter};

/// Hosts for IQOption API
pub struct EnviromentHosts {
    pub api: &'static str,
    pub auth: &'static str,
}

/// Debug implementation for EnviromentHosts
impl Debug for EnviromentHosts {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnviromentHosts")
            .field("api", &self.api)
            .field("auth", &self.auth)
            .finish()
    }
}

/// Implementation of methods for EnviromentHosts
impl PartialEq for EnviromentHosts {
    fn eq(&self, other: &Self) -> bool {
        self.api == other.api && self.auth == other.auth
    }
}

/// Enviroment for IQOption API
pub enum Enviroment {
    Simple,
}

/// Default enum value for Enviroment
impl Default for Enviroment {
    fn default() -> Self {
        const ENV: Enviroment = Enviroment::Simple;

        ENV
    }
}

/// Debug implementation for Enviroment
impl Debug for Enviroment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Implementation of methods for Enviroment
impl PartialEq for Enviroment {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

/// Implementation of methods for Enviroment
impl Enviroment {
    /// String representation of Enviroment
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            Enviroment::Simple => "Simple",
        }
    }

    /// User-Agent for use in IQOption API.
    /// We are using a static user-agent for this purpose as IQ Option does not
    /// impose significant restrictions in this regard.
    #[inline]
    pub fn user_agent(&self) -> &str {
        match self {
            Enviroment::Simple => "Mozilla/5.0",
        }
    }
}

/// Static methods for Enviroment
impl Enviroment {
    /// Static Hosts for IQOption API
    pub fn hosts() -> EnviromentHosts {
        EnviromentHosts {
            api: "iqoption.com",
            auth: "auth.iqoption.com",
        }
    }
}

/// Unit tests for Enviroment
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_enviroment_default() {
        assert_eq!(Enviroment::default(), Enviroment::Simple);
    }

    #[test]
    fn test_enviroment_as_str() {
        assert_eq!(Enviroment::Simple.as_str(), "Simple");
    }

    #[test]
    fn test_enviroment_user_agent() {
        assert_eq!(Enviroment::Simple.user_agent(), "Mozilla/5.0");
    }

    #[test]
    fn test_enviroment_hosts_eq() {
        assert_eq!(
            Enviroment::hosts(),
            EnviromentHosts {
                api: "iqoption.com",
                auth: "auth.iqoption.com",
            }
        );
    }

    #[test]
    fn test_enviroment_hosts_debug() {
        assert_eq!(
            format!("{:?}", Enviroment::hosts()),
            "EnviromentHosts { api: \"iqoption.com\", auth: \"auth.iqoption.com\" }"
        );
    }

    #[test]
    fn test_enviroment_debug() {
        assert_eq!(format!("{:?}", Enviroment::Simple), "Simple");
    }
}
