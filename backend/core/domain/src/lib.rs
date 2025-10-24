#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
pub struct Event {
    pub id: Uuid,
    pub name: ::aliases::string::String,
    pub description: ::aliases::string::String,
    pub location: ::aliases::string::String,
    pub timestamp: ::aliases::time::Timestamp,

    pub status: EventStatus,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
pub enum EventStatus {
    Created {
        manager_id: Uuid,
    },
    Approved {
        administrator_id: Uuid,
    },
    Completed {
        timestamp: ::aliases::time::Timestamp,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
pub struct Channel {
    pub id: Uuid,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
pub struct Post {
    pub id: Uuid,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
pub struct User {
    pub username: Username,
    pub email: Email,
    pub password: PasswordDigest,

    pub role: UserRole,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum UserRole {
    Volunteer,
    EventManager,
    Administrator,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::core::cmp::Eq, ::core::cmp::PartialEq, ::core::cmp::Ord, ::core::cmp::PartialOrd)]
pub struct Uuid([u8; 16]);

#[::bon::bon]
impl Uuid {
    #[builder]
    pub fn new(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl ::core::ops::Deref for Uuid {
    type Target = [u8; 16];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
pub struct Username(::aliases::string::String);

#[::bon::bon]
impl Username {
    #[builder(on(::aliases::string::String, into))]
    pub fn new(value: ::aliases::string::String) -> ::core::result::Result<Self, UsernameBuilderError> {
        let value = Self::normalize(value);
        Self::validate(value).map(Self)
    }

    fn normalize(value: ::aliases::string::String) -> ::aliases::string::String {
        let trimmed = value.trim();

        if value.len() == trimmed.len() {
            value
        } else {
            trimmed.chars().collect()
        }
    }

    fn validate(value: ::aliases::string::String) -> ::core::result::Result<::aliases::string::String, UsernameBuilderError> {
        let regex = ::aliases::regex!("^[a-z0-9_-]{4,16}$");

        if !regex.is_match(&value) {
            ::core::result::Result::Err(UsernameBuilderError::InvalidFormat)
        } else {
            ::core::result::Result::Ok(value)
        }
    }
}

impl ::core::ops::Deref for Username {
    type Target = ::aliases::string::String;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[derive(::thiserror::Error)]
pub enum UsernameBuilderError {
    #[error("Invalid username format: must be between 4 and 16 characters; lowercase letters, digits, underscores, or hyphens only")]
    InvalidFormat,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
pub struct Email(::aliases::string::String);

#[::bon::bon]
impl Email {
    #[builder(on(::aliases::string::String, into))]
    pub fn new(value: ::aliases::string::String) -> ::core::result::Result<Self, EmailBuilderError> {
        let value = Self::normalize(value);
        Self::validate(value).map(Self)
    }

    fn normalize(value: ::aliases::string::String) -> ::aliases::string::String {
        let trimmed = value.trim();

        if value.len() == trimmed.len() && !trimmed.chars().any(|char| char.is_control()) && !trimmed.chars().any(|char| char.is_uppercase()) {
            value
        } else {
            trimmed.chars()
                .filter(|char| !char.is_control())
                .flat_map(|char| char.to_lowercase())
                .collect()
        }
    }

    fn validate(value: ::aliases::string::String) -> ::core::result::Result<::aliases::string::String, EmailBuilderError> {
        // RFC 5322 Official Standard
        // https://emailregex.com/
        let regex = ::aliases::regex!(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#);

        if !regex.is_match(&value) {
            ::core::result::Result::Err(EmailBuilderError::InvalidFormat)
        } else {
            ::core::result::Result::Ok(value)
        }
    }
}

impl ::core::ops::Deref for Email {
    type Target = ::aliases::string::String;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[derive(::thiserror::Error)]
pub enum EmailBuilderError {
    #[error("Invalid email format: does not comply with RFC 5322")]
    InvalidFormat,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
pub struct Password(::aliases::string::String);

#[::bon::bon]
impl Password {
    #[builder(on(::aliases::string::String, into))]
    pub fn new(value: ::aliases::string::String) -> ::core::result::Result<Self, PasswordBuilderError> {
        let value = Self::normalize(value);
        Self::validate(value).map(Self)
    }

    fn normalize(value: ::aliases::string::String) -> ::aliases::string::String {
        let trimmed = value.trim();

        if value.len() == trimmed.len() {
            value
        } else {
            trimmed.chars().collect()
        }
    }

    fn validate(value: ::aliases::string::String) -> ::core::result::Result<::aliases::string::String, PasswordBuilderError> {
        let regex = ::aliases::regex!("^\\w{8,32}$");

        if !regex.is_match(&value) {
            ::core::result::Result::Err(PasswordBuilderError::InvalidFormat)
        } else {
            ::core::result::Result::Ok(value)
        }
    }
}

impl ::core::ops::Deref for Password {
    type Target = ::aliases::string::String;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
#[derive(::thiserror::Error)]
pub enum PasswordBuilderError {
    #[error("Invalid password format: must be between 8 and 32 characters")]
    InvalidFormat,
}

pub type PasswordDigest = ::aliases::string::String;
