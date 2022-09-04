use std::{borrow::Cow, error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScScrapingError<'a> {
    Citizen { message: Cow<'a, str> },
    Organization { message: Cow<'a, str> },
}

impl<'a> ScScrapingError<'a> {
    pub fn citizen<S>(message: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::Citizen {
            message: message.into(),
        }
    }

    pub fn organization<S>(message: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::Organization {
            message: message.into(),
        }
    }
}

impl<'a> Error for ScScrapingError<'a> {}
impl<'a> Display for ScScrapingError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Citizen { message } => write!(f, "{message}"),
            Self::Organization { message } => write!(f, "{message}"),
        }
    }
}
