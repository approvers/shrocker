use std::str::FromStr;

pub struct Username(String);

#[derive(thiserror::Error, Debug)]
pub enum UsernameParseError {
    #[error("Too long or short: must be between 2 to 127")]
    Length,

    #[error("Contains unusable character: must be one of /[A-Za-z0-9]/, `-`, `_`")]
    UnusableChar
}

impl FromStr for Username {
    type Err = UsernameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(2..=127).contains(&s.len()) {
            return Err(UsernameParseError::Length);
        }

        if !s.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_') {
            return Err(UsernameParseError::UnusableChar);
        }

        Ok(Username(s.to_string()))
    }
}

impl ToString for Username {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct SSHPublicKey(String);

impl SSHPublicKey {
    pub(crate) fn new(raw: &str) -> Self {
        Self(raw.to_string())
    }
}

impl ToString for SSHPublicKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
