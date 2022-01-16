#[derive(Debug)]
pub enum PodinfoError {
    ConfigationError(String),
}

impl From<envconfig::Error> for PodinfoError {
    fn from(e: envconfig::Error) -> Self {
        match e {
            envconfig::Error::ParseError { name } => {
                PodinfoError::ConfigationError(format!("Failed to parse environment var {}!", name))
            }
            envconfig::Error::EnvVarMissing { name } => {
                PodinfoError::ConfigationError(format!("Missing environment var: {}", name))
            }
        }
    }
}
