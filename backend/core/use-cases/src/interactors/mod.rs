#![allow(clippy::option_map_unit_fn)]

mod create_comment;
mod create_event;
mod create_post;
mod create_reaction;
mod export_events;
mod export_volunteers;
mod moderate_event;
mod moderate_event_registration;
mod moderate_user;
mod remove_comment;
mod remove_event;
mod remove_post;
mod remove_reaction;
mod sign_in;
mod sign_up;
mod subscribe_to_event;
mod unsubscribe_from_event;
mod update_comment;
mod update_event;
mod update_post;
mod view_event_channel;
mod view_event_history;
mod view_event_recommendation;
mod view_event_volunteers;
mod view_events;
mod view_post;
mod view_published_events;
mod view_users;
mod view_event;
mod view_published_event;
mod view_self_profile;
mod view_user;

pub use self::create_comment::*;
pub use self::create_event::*;
pub use self::create_post::*;
pub use self::create_reaction::*;
pub use self::export_events::*;
pub use self::export_volunteers::*;
pub use self::moderate_event::*;
pub use self::moderate_event_registration::*;
pub use self::moderate_user::*;
pub use self::remove_comment::*;
pub use self::remove_event::*;
pub use self::remove_post::*;
pub use self::remove_reaction::*;
pub use self::sign_in::*;
pub use self::sign_up::*;
pub use self::subscribe_to_event::*;
pub use self::unsubscribe_from_event::*;
pub use self::update_comment::*;
pub use self::update_event::*;
pub use self::update_post::*;
pub use self::view_event_channel::*;
pub use self::view_event_history::*;
pub use self::view_event_recommendation::*;
pub use self::view_event_volunteers::*;
pub use self::view_events::*;
pub use self::view_post::*;
pub use self::view_published_events::*;
pub use self::view_users::*;
pub use self::view_event::*;
pub use self::view_published_event::*;
pub use self::view_self_profile::*;
pub use self::view_user::*;

/// Assumes: **(1)** `$ok` is of type `OkResponse`; **(2)** `type Response =
/// ::core::result::Result<OkResponse, _>`; **(3)** crate `axiom` is in scope.
macro_rules! ok {
    ($($ok:tt)*) => {
        ::axiom::result::Fallible::Ok(Response::Ok($($ok)*))
    };
}

/// Assumes: **(1)** `$errs` is of type
/// `::std::vec::Vec<ErrResponse>`; **(2)** `type Response =
/// ::core::result::Result<_, ::std::vec::Vec<ErrResponse>>`; **(3)** crate
/// `axiom` is in scope.
macro_rules! errs {
    ($($errs:tt)*) => {
        ::axiom::result::Fallible::Ok(Response::Err($($errs)*))
    };
}

/// Assumes: **(1)** `$err` is of type `ErrResponse`; **(2)** `type Response =
/// ::core::result::Result<_, ::std::vec::Vec<ErrResponse>>`; **(3)** crate
/// `axiom` is in scope.
macro_rules! err {
    ($($err:tt)*) => {
        ::axiom::result::Fallible::Ok(Response::Err(::std::vec![ErrResponse::$($err)*]))
    }
}

pub(in crate::interactors) use err;
pub(in crate::interactors) use errs;
pub(in crate::interactors) use ok;
