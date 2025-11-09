use ::async_trait::async_trait;
#[cfg(feature = "wasm-bindings")]
use ::wasm_bindgen::prelude::*;

#[async_trait]
pub trait SignInBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignInRequest)
        -> ::aliases::result::Fallible<SignInResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInRequest {
    pub username_or_email: ::aliases::string::String,
    pub password: ::aliases::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type SignInResponse = ::core::result::Result<SignInOkResponse, ::std::vec::Vec<SignInErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignInOkResponse {
    pub user_role: self::models::UserRole,
    pub token: ::aliases::string::String,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignInErrResponse {
    #[error("User with username `{username}` not found")]
    UsernameNotFound {
        username: ::aliases::string::String,
    },

    #[error("User with email `{email}` not found")]
    EmailNotFound {
        email: ::aliases::string::String,
    },

    #[error("{} or {}", 0.0, 0.1)]
    UsernameOrEmailInvalid(
        #[cfg_attr(feature = "serde", serde(skip))]
        (::domain::UsernameBuilderError, ::domain::EmailBuilderError),
    ),

    #[error(transparent)]
    PasswordInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::PasswordBuilderError),

    #[error("Passwords do not match")]
    PasswordMismatch,
}

#[async_trait]
pub trait SignUpBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: SignUpRequest)
        -> ::aliases::result::Fallible<SignUpResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct SignUpRequest {
    pub user_role: self::models::UserRole,

    pub username: ::aliases::string::String,
    pub email: ::aliases::string::String,
    pub password: ::aliases::string::String,

    pub first_name: ::aliases::string::String,
    pub last_name: ::aliases::string::String,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type SignUpResponse = ::core::result::Result<SignUpOkResponse, ::std::vec::Vec<SignUpErrResponse>>;

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type SignUpOkResponse = ();

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum SignUpErrResponse {
    #[error(transparent)]
    UsernameInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::UsernameBuilderError),

    #[error(transparent)]
    EmailInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::EmailBuilderError),

    #[error(transparent)]
    PasswordInvalid(#[from] #[cfg_attr(feature = "serde", serde(skip))] ::domain::PasswordBuilderError),

    #[error("User with username `{username}` already exists")]
    UsernameAlreadyExists {
        username: ::aliases::string::String,
    },

    #[error("User with email `{email}` already exists")]
    EmailAlreadyExists {
        email: ::aliases::string::String,
    },
}

#[async_trait]
pub trait ViewEventRecommendationBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewEventRecommendationRequest)
        -> ::aliases::result::Fallible<ViewEventRecommendationResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewEventRecommendationRequest {
    pub token: ::aliases::string::String,
    pub r#type: self::models::EventRecommendationType,
    pub limit: usize,
}

#[cfg_attr(feature = "wasm-bindings", ::tsify::declare)]
pub type ViewEventRecommendationResponse = ::core::result::Result<ViewEventRecommendationOkResponse, ::std::vec::Vec<ViewEventRecommendationErrResponse>>;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewEventRecommendationOkResponse {
    pub events: ::std::vec::Vec<self::models::EventPreview>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum ViewEventRecommendationErrResponse {
    #[error("Invalid or expired authentication token")]
    AuthenticationTokenInvalid,
}

// Volunteer's
#[async_trait]
pub trait ViewEventsByTimestampBoundary {

}

#[async_trait]
pub trait ViewEventsByCategoryBoundary {
    
}

#[async_trait]
pub trait SubscribeToEventBoundary {
    
}

#[async_trait]
pub trait UnsubscribeFromEventBoundary {
    
}

#[async_trait]
pub trait ViewEventHistoryBoundary {

}

// Event manager's
#[async_trait]
pub trait CreateEventBoundary {

}

#[async_trait]
pub trait UpdateEventBoundary {

}

#[async_trait]
pub trait RemoveEventBoundary {

}

#[async_trait]
pub trait AcceptVolunteerEventRegistrationBoundary {
    
}

#[async_trait]
pub trait DeclineVolunteerEventRegistrationBoundary {
    
}

#[async_trait]
pub trait CompleteVolunteerEventRegistrationBoundary {

}

#[async_trait]
pub trait ViewEventVolunteersBoundary {

}

// Administrator's
#[async_trait]
pub trait ApproveEventBoundary {
    
}

#[async_trait]
pub trait RejectEventBoundary {

}

#[async_trait]
pub trait ViewNonAdministratorUsersBoundary {
    
}

#[async_trait]
pub trait ViewNonAdministratorUserBoundary {

}

#[async_trait]
pub trait SuspendNonAdministratorUserBoundary {

}

#[async_trait]
pub trait ReinstateNonAdministratorUserBoundary {
    
}

#[async_trait]
pub trait ExportEventsBoundary {

}

#[async_trait]
pub trait ExportVolunteersBoundary {

}

// Common
#[async_trait]
pub trait ViewEventChannelBoundary {

}

#[async_trait]
pub trait CreateEventChannelPostBoundary {

}

#[async_trait]
pub trait CreateEventChannelPostReactionBoundary {

}

#[async_trait]
pub trait CreateEventChannelPostCommentBoundary {

}

// To be exposed to `tsify`/`wasm-bindgen`
pub mod models {
    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub enum EventRecommendationType {
        RecentlyPublished,
        RecentlyPosted,
        Trending,
    }

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub enum ExportFormat {
        CSV,
        JSON,
    }

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
    #[builder(on(_, into))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub struct Event {
        pub id: ::aliases::string::String,
        pub channel_id: ::core::option::Option<::aliases::string::String>,

        pub statuses: ::std::vec::Vec<self::EventStatus>,

        pub name: ::aliases::string::String,
        pub description: ::aliases::string::String,
        pub categories: ::std::vec::Vec<::aliases::string::String>,
        pub location: ::aliases::string::String,
    }

    #[derive(::core::fmt::Debug, ::core::clone::Clone)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case", rename_all_fields = "kebab-case"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub enum EventStatus {
        Created {
            created_by_manager_id: ::aliases::string::String,
        },
        Approved {
            approved_by_administrator_id: ::aliases::string::String,
            approved_at: ::aliases::time::Timestamp,
        },
        Rejected {
            rejected_by_administrator_id: ::aliases::string::String,
            rejected_at: ::aliases::time::Timestamp,
        },
        Completed {
            completed_by_manager_id: ::aliases::string::String,
            completed_at: ::aliases::time::Timestamp,
        },
    }

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
    #[builder(on(_, into))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub struct EventPreview {
        pub id: ::aliases::string::String,

        #[builder(with = |value: ::std::vec::Vec<impl ::core::convert::Into<self::EventStatusPreview>>| value.into_iter().map(::core::convert::Into::into).collect())]
        pub statuses: ::std::vec::Vec<self::EventStatusPreview>,

        pub name: ::aliases::string::String,
        pub categories: ::std::vec::Vec<::aliases::string::String>,
    }

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case", rename_all_fields = "kebab-case"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub enum EventStatusPreview {
        Created,
        Approved,
        Rejected,
        Completed,
    }

    impl ::core::convert::From<::domain::EventStatus> for EventStatusPreview {
        fn from(value: ::domain::EventStatus) -> Self {
            match value {
                ::domain::EventStatus::Created { .. } => Self::Created,
                ::domain::EventStatus::Approved { .. } => Self::Approved,
                ::domain::EventStatus::Rejected { .. } => Self::Rejected,
                ::domain::EventStatus::Completed { .. } => Self::Completed,
            }
        }
    }

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
    pub enum UserRole {
        Volunteer,
        EventManager,
        Administrator,
    }

    impl ::core::convert::From<::domain::UserRole> for UserRole {
        fn from(value: ::domain::UserRole) -> Self {
            match value {
                ::domain::UserRole::Volunteer => Self::Volunteer,
                ::domain::UserRole::EventManager => Self::EventManager,
                ::domain::UserRole::Administrator => Self::Administrator,
            }
        }
    }

    impl ::core::convert::From<UserRole> for ::domain::UserRole {
        fn from(value: UserRole) -> Self {
            match value {
                UserRole::Volunteer => Self::Volunteer,
                UserRole::EventManager => Self::EventManager,
                UserRole::Administrator => Self::Administrator,
            }
        }
    }
}
