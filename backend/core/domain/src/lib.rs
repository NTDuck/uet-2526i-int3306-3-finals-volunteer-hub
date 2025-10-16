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
#[derive(::bon::Builder)]
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

#[derive(::bon::Builder)]
pub struct EventChannel {
    pub id: Uuid,

}

// #[derive(::bon::Builder)]
// pub struct Post {
//     pub id: Uuid,
//     pub comments: ...,
//     pub likes: ...,
// }

#[derive(::bon::Builder)]
pub struct Volunteer {
    pub email: ::aliases::string::String,
    pub password_digest: ::aliases::string::String,
}

#[derive(::bon::Builder)]
pub struct EventManager {
    pub email: ::aliases::string::String,
    pub password_digest: ::aliases::string::String,
}

#[derive(::bon::Builder)]
pub struct Administrator {
    pub email: ::aliases::string::String,
    pub password_digest: ::aliases::string::String,
}

// Note that validating emial & password is domain logic!
