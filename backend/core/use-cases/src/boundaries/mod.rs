mod accept_event_registration;
mod approve_event;
mod complete_event_registration;
mod view_event_channel;
mod view_post;
mod create_comment;
mod create_event;
mod create_post;
mod create_reaction;
mod decline_event_registration;
mod export_events;
mod export_volunteers;
mod unsuspend_non_admin_user;
mod reject_event;
mod remove_event;
mod sign_in;
mod sign_up;
mod subscribe_to_event;
mod suspend_non_admin_user;
mod unsubscribe_from_event;
mod update_event;
mod view_event_history;
mod view_event_recommendation;
mod view_event_volunteers;
mod view_non_admin_user;
mod view_non_admin_users;
mod view_published_events;
mod view_event;
mod update_event_registration;

use serde::ser::SerializeMap;

pub use self::accept_event_registration::*;
pub use self::approve_event::*;
pub use self::complete_event_registration::*;
pub use self::view_event_channel::*;
pub use self::view_post::*;
pub use self::create_comment::*;
pub use self::create_event::*;
pub use self::create_post::*;
pub use self::create_reaction::*;
pub use self::decline_event_registration::*;
pub use self::export_events::*;
pub use self::export_volunteers::*;
pub use self::unsuspend_non_admin_user::*;
pub use self::reject_event::*;
pub use self::remove_event::*;
pub use self::sign_in::*;
pub use self::sign_up::*;
pub use self::subscribe_to_event::*;
pub use self::suspend_non_admin_user::*;
pub use self::unsubscribe_from_event::*;
pub use self::update_event::*;
pub use self::view_event_history::*;
pub use self::view_event_recommendation::*;
pub use self::view_event_volunteers::*;
pub use self::view_non_admin_user::*;
pub use self::view_non_admin_users::*;
pub use self::view_published_events::*;
pub use self::view_event::*;
pub use self::update_event_registration::*;

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "serde", derive(::axiom::SerializableError))]
// #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case", rename_all_fields = "kebab-case", tag = "error"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(into_wasm_abi))]
pub enum __GenericErrResponse {
    #[cfg_attr(feature = "serde", error(message = "message is foo"))]
    Foo,

    #[cfg_attr(feature = "serde", error(message = "message is {1}"))]
    Bar(String, String),

    #[cfg_attr(feature = "serde", error(message = "message is {foo} {bar}"))]
    Baz {
        foo: String,
        bar: String,
    },
}

// impl ::serde::ser::Serialize for __GenericErrResponse {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: ::serde::ser::Serializer,
//     {
//         fn as_error_code(this: &__GenericErrResponse) -> &'static str {
//             match this {
//                 __GenericErrResponse::Foo => "foo",
//                 __GenericErrResponse::Bar(_, _) => "bar",
//                 __GenericErrResponse::Baz { .. } => "baz",
//             }
//         }

//         fn to_message(this: &__GenericErrResponse) -> String {
//             to_data(this).to_string()
//         }

//         fn to_data<'a>(this: &'a __GenericErrResponse) -> __GenericErrResponseRepr<'a> {
//             match this {
//                 __GenericErrResponse::Foo => __GenericErrResponseRepr::Foo,
//                 __GenericErrResponse::Bar(arg0, arg1) => __GenericErrResponseRepr::Bar(arg0, arg1),
//                 __GenericErrResponse::Baz { foo, bar } => __GenericErrResponseRepr::Baz { foo, bar },
//             }
//         }

//         let mut map = serializer.serialize_map(::core::option::Option::Some(3))?;

//         map.serialize_entry("error", as_error_code(&self))?;
//         map.serialize_entry("message", &to_message(self))?;
//         map.serialize_entry("data", &to_data(&self))?;

//         map.end()
//     }
// }

// #[derive(::serde::Serialize, ::core::fmt::Debug, ::thiserror::Error)]
// #[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case", untagged)]
// enum __GenericErrResponseRepr<'__serializable_error_repr> {
//     #[error("message is foo")]
//     Foo,
//     #[error("message is {1}")]
//     Bar(&'__serializable_error_repr String, &'__serializable_error_repr String),
//     #[error("message is {foo} {bar}")]
//     Baz { foo: &'__serializable_error_repr String, bar: &'__serializable_error_repr String },
// }
