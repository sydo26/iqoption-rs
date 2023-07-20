use crate::{Enviroment, Session};

/// Client for IQOption API
pub struct IQOptionClient {
    /// Enviroment used in IQOption API
    pub enviroment: Enviroment,

    /// Session for IQOption API
    pub session: Session,
}

impl IQOptionClient {
    pub fn new(session: Session, enviroment: Enviroment) -> Self {
        IQOptionClient {
            enviroment,
            session,
        }
    }
}

/// Implementation of methods for IQOptionClient
impl IQOptionClient {}

/// Unit tests for IQOptionClient
#[cfg(test)]
mod tests {}
