#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
pub struct Event {
    pub id: Uuid,
    pub channel_id: ::core::option::Option<Uuid>,

    pub statuses: ::std::vec::Vec<EventStatus>,

    pub name: EventName,
    pub description: EventDescription,
    pub categories: ::std::vec::Vec<EventCategory>,
    pub location: EventLocation,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum EventStatus {
    Created {
        created_by_manager_id: Uuid,
    },
    Approved {
        approved_by_administrator_id: Uuid,
        approved_at: ::axiom::time::Timestamp,
    },
    Rejected {
        rejected_by_administrator_id: Uuid,
        rejected_at: ::axiom::time::Timestamp,
    },
    Completed {
        completed_by_manager_id: Uuid,
        completed_at: ::axiom::time::Timestamp,
    },
}

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::axiom::Verifiable)]
#[verifiable(regex = r#"^.{4,64}$"#, hint = "must be between 4 and 64 characters")]
pub struct EventName(::axiom::string::String);

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::axiom::Verifiable)]
#[verifiable(regex = r#"^.{0,512}$"#, hint = "must be at most 512 characters")]
pub struct EventDescription(::axiom::string::String);

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::axiom::Verifiable)]
#[verifiable(regex = r#"^[a-zA-Z0-9 ]{2,32}$"#, hint = "must be between 2 and 32 characters; letters, digits, or spaces only")]
pub struct EventCategory(::axiom::string::String);

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::axiom::Verifiable)]
#[verifiable(regex = r#"^.{4,128}$"#, hint = "must be between 4 and 128 characters")]
pub struct EventLocation(::axiom::string::String);

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

    pub content: EventChannelPostCommentContent,
}

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::axiom::Verifiable)]
#[verifiable(regex = r#"^.{1,256}$"#, hint = "must be between 1 and 256 characters")]
pub struct EventChannelPostCommentContent(::axiom::string::String);

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
pub struct User {
    pub id: Uuid,
    pub role: UserRole,

    pub username: Username,
    pub email: Email,
    pub password: PasswordDigest,

    pub full_name: FullName,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum UserRole {
    Volunteer,
    EventManager,
    Administrator,
}

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::Eq, ::core::cmp::PartialEq, ::core::hash::Hash, ::axiom::Verifiable)]
#[verifiable(regex = "^[a-z0-9_-]{4,16}$", hint = "must be between 4 and 16 characters; lowercase letters, digits, underscores (`_`), or hyphens (`-`) only")]
pub struct Username(::axiom::string::String);

// RFC 5322 Official Standard
// https://emailregex.com/
#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::Eq, ::core::cmp::PartialEq, ::core::hash::Hash, ::axiom::Verifiable)]
#[verifiable(regex = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#, hint = "must comply with RFC 5322")]
pub struct Email(::axiom::string::String);

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::axiom::Verifiable)]
#[verifiable(regex = "^.{8,32}$", hint = "must be between 8 and 32 characters")]
pub struct Password(::axiom::string::String);

pub type PasswordDigest = ::axiom::string::String;

#[repr(transparent)]
#[derive(::core::fmt::Debug, ::core::clone::Clone, ::axiom::Verifiable)]
#[verifiable(regex = r"^[A-Za-z][A-Za-z\s'-]{4,128}[A-Za-z]$", hint = "must be between 4 and 128 characters; letters, spaces, apostrophes (`'`), or hyphens (`-`) only")]
pub struct FullName(::axiom::string::String);

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
        pending_at: ::axiom::time::Timestamp,
    },
    Withdrawn {
        withdrawn_at: ::axiom::time::Timestamp,
    },
    Accepted {
        accepted_by_manager_id: Uuid,
        accepted_at: ::axiom::time::Timestamp,
    },
    Declined {
        declined_by_manager_id: Uuid,
        declined_at: ::axiom::time::Timestamp,
    },
    Completed {
        completed_by_manager_id: Uuid,
        completed_at: ::axiom::time::Timestamp,
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
