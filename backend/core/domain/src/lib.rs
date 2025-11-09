#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
pub struct Event {
    pub id: Uuid,
    pub channel_id: ::core::option::Option<Uuid>,

    pub statuses: ::std::vec::Vec<EventStatus>,

    pub name: ::aliases::string::String,
    pub description: ::aliases::string::String,
    pub categories: ::std::vec::Vec<::aliases::string::String>,
    pub location: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum EventStatus {
    Created {
        created_by_manager_id: Uuid,
    },
    Approved {
        approved_by_administrator_id: Uuid,
        approved_at: ::aliases::time::Timestamp,
    },
    Rejected {
        rejected_by_administrator_id: Uuid,
        rejected_at: ::aliases::time::Timestamp,
    },
    Completed {
        completed_by_manager_id: Uuid,
        completed_at: ::aliases::time::Timestamp,
    },
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
pub struct EventChannel {
    pub id: Uuid,
    pub event_id: Uuid,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
pub struct EventChannelPost {
    pub id: Uuid,
    pub volunteer_id: Uuid,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
pub struct EventChannelPostReaction {
    pub id: Uuid,
    pub volunteer_id: Uuid,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
pub struct EventChannelPostComment {
    pub id: Uuid,
    pub volunteer_id: Uuid,

    pub content: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
pub struct User {
    pub id: Uuid,
    pub role: UserRole,

    pub username: Username,
    pub email: Email,
    pub password: PasswordDigest,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum UserRole {
    Volunteer,
    EventManager,
    Administrator,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
pub struct EventRegistration {
    pub event_id: Uuid,
    pub volunteer_id: Uuid,

    pub statuses: ::std::vec::Vec<EventRegistrationStatus>,
}

/// Possible lifecycles
/// 1. `Pending` (volunteer subscribes to event) -> `Accepted` (event manager accepts registration) -> `Completed` (event manager updates registration status after event completion)
/// 2. `Pending` (volunteer subscribes to event) -> `Declined` (event manager declines registration)
/// 3. `Pending` (volunteer subscribes to event) -> `Withdrawn` (volunteer unsubscribes from event)
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum EventRegistrationStatus {
    Pending {
        pending_at: ::aliases::time::Timestamp,
    },
    Withdrawn {
        withdrawn_at: ::aliases::time::Timestamp,
    },
    Accepted {
        accepted_by_manager_id: Uuid,
        accepted_at: ::aliases::time::Timestamp,
    },
    Declined {
        declined_by_manager_id: Uuid,
        declined_at: ::aliases::time::Timestamp,
    },
    Completed {
        completed_by_manager_id: Uuid,
        completed_at: ::aliases::time::Timestamp,
    },
}

#[derive(
    ::core::fmt::Debug,
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::cmp::Eq,
    ::core::cmp::PartialEq,
    ::core::cmp::Ord,
    ::core::cmp::PartialOrd,
    ::core::hash::Hash
)]
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

#[derive(
    ::core::fmt::Debug,
    ::core::clone::Clone,
    ::core::cmp::Eq,
    ::core::cmp::PartialEq,
    ::core::cmp::Ord,
    ::core::cmp::PartialOrd,
    ::core::hash::Hash
)]
pub struct Username(::aliases::string::String);

#[::bon::bon]
impl Username {
    #[builder(on(_, into))]
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

    fn validate(
        value: ::aliases::string::String,
    ) -> ::core::result::Result<::aliases::string::String, UsernameBuilderError> {
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

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::core::default::Default, ::thiserror::Error)]
pub enum UsernameBuilderError {
    #[default]
    #[error(
        "Invalid username format: must be between 4 and 16 characters; lowercase letters, digits, underscores, or \
         hyphens only"
    )]
    InvalidFormat,
}

#[derive(
    ::core::fmt::Debug,
    ::core::clone::Clone,
    ::core::cmp::Eq,
    ::core::cmp::PartialEq,
    ::core::cmp::Ord,
    ::core::cmp::PartialOrd,
    ::core::hash::Hash
)]
pub struct Email(::aliases::string::String);

#[::bon::bon]
impl Email {
    #[builder(on(_, into))]
    pub fn new(value: ::aliases::string::String) -> ::core::result::Result<Self, EmailBuilderError> {
        let value = Self::normalize(value);
        Self::validate(value).map(Self)
    }

    fn normalize(value: ::aliases::string::String) -> ::aliases::string::String {
        let trimmed = value.trim();

        if value.len() == trimmed.len()
            && !trimmed.chars().any(|char| char.is_control())
            && !trimmed.chars().any(|char| char.is_uppercase())
        {
            value
        } else {
            trimmed
                .chars()
                .filter(|char| !char.is_control())
                .flat_map(|char| char.to_lowercase())
                .collect()
        }
    }

    fn validate(
        value: ::aliases::string::String,
    ) -> ::core::result::Result<::aliases::string::String, EmailBuilderError> {
        // RFC 5322 Official Standard
        // https://emailregex.com/
        let regex = ::aliases::regex!(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
        );

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

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::core::default::Default, ::thiserror::Error)]
pub enum EmailBuilderError {
    #[default]
    #[error("Invalid email format: does not comply with RFC 5322")]
    InvalidFormat,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
pub struct Password(::aliases::string::String);

#[::bon::bon]
impl Password {
    #[builder(on(_, into))]
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

    fn validate(
        value: ::aliases::string::String,
    ) -> ::core::result::Result<::aliases::string::String, PasswordBuilderError> {
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

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::core::default::Default, ::thiserror::Error)]
pub enum PasswordBuilderError {
    #[default]
    #[error("Invalid password format: must be between 8 and 32 characters")]
    InvalidFormat,
}

pub type PasswordDigest = ::aliases::string::String;
