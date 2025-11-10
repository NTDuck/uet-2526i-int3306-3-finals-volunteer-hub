use ::async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn view_recently_approved(self: ::std::sync::Arc<Self>, limit: usize) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;
    async fn view_recently_posted(self: ::std::sync::Arc<Self>, limit: usize) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;
    async fn view_trending(self: ::std::sync::Arc<Self>, limit: usize) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;

    async fn view(self: ::std::sync::Arc<Self>, filter: self::models::ViewEventsFilter) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;
}

#[async_trait]
pub trait UserRepository {
    async fn save(self: ::std::sync::Arc<Self>, user: ::domain::User) -> ::aliases::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_email(
        self: ::std::sync::Arc<Self>, email: ::domain::Email,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_id(id).await?.is_some())
    }

    async fn contains_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_username(username).await?.is_some())
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::domain::Email) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_email(email).await?.is_some())
    }
}

#[async_trait]
pub trait UuidGenerator {
    async fn generate(self: ::std::sync::Arc<Self>) -> ::aliases::result::Fallible<::domain::Uuid>;

    async fn get_timestamp(
        self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid,
    ) -> ::aliases::result::Fallible<::aliases::time::Timestamp>;
}

#[async_trait]
pub trait UuidCodec {
    async fn format(self: ::std::sync::Arc<Self>, uuid: ::domain::Uuid) -> ::aliases::result::Fallible<::aliases::string::String>;
    async fn parse(self: ::std::sync::Arc<Self>, uuid: ::aliases::string::String) -> ::aliases::result::Fallible<::domain::Uuid>;
}

#[async_trait]
pub trait AuthenticationTokenGenerator {
    async fn generate(
        self: ::std::sync::Arc<Self>, payload: self::models::AuthenticationTokenPayload,
    ) -> ::aliases::result::Fallible<::aliases::string::String>;

    async fn get_payload(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<self::models::AuthenticationTokenPayload>>;

    async fn get_user_id(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::Uuid>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|payload| payload.user_id))
    }

    async fn get_user_role(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::UserRole>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|payload| payload.user_role))
    }

    async fn get_expiry_timestamp(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<::aliases::time::Timestamp>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|payload| payload.expiry_timestamp))
    }

    async fn verify(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.is_some())
    }
}

#[async_trait]
pub trait PasswordHasher {
    async fn hash(
        self: ::std::sync::Arc<Self>, password: ::domain::Password,
    ) -> ::aliases::result::Fallible<::domain::PasswordDigest>;

    async fn verify(
        self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest,
    ) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.hash(password).await? == digest)
    }
}

// Not to be exposed to `tsify`/`wasm-bindgen`
pub mod models {
    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
    #[builder(on(_, into))]
    pub struct ViewEventsFilter {
        #[builder(with = |value: ::std::vec::Vec<impl ::core::convert::Into<self::EventStatusPreview>>| value.into_iter().map(::core::convert::Into::into).collect())]
        pub statuses: ::std::vec::Vec<self::EventStatusPreview>,

        pub name: ::core::option::Option<::aliases::string::String>,
        pub description: ::core::option::Option<::aliases::string::String>,
        pub category: ::core::option::Option<::aliases::string::String>,
        pub location: ::core::option::Option<::aliases::string::String>,

        pub timestamps: ::core::ops::Range<::core::option::Option<::aliases::time::Timestamp>>,
    }

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
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

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::bon::Builder)]
    #[builder(on(_, into))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(
        feature = "serde",
        serde(from = "__AuthenticationTokenPayload", into = "__AuthenticationTokenPayload", rename_all = "camelCase")
    )]
    pub struct AuthenticationTokenPayload {
        pub user_id: ::domain::Uuid,
        pub user_role: ::domain::UserRole,
        pub expiry_timestamp: ::aliases::time::Timestamp,
    }

    #[cfg(feature = "serde")]
    #[derive(::serde::Serialize, ::serde::Deserialize, ::bon::Builder)]
    #[builder(on(_, into))]
    struct __AuthenticationTokenPayload {
        user_id: __Uuid,
        user_role: crate::boundaries::models::UserRole,
        expiry_timestamp: ::aliases::time::Timestamp,
    }

    #[cfg(feature = "serde")]
    impl ::core::convert::From<__AuthenticationTokenPayload> for AuthenticationTokenPayload {
        fn from(value: __AuthenticationTokenPayload) -> Self {
            Self::builder()
                .user_id(value.user_id)
                .user_role(value.user_role)
                .expiry_timestamp(value.expiry_timestamp)
                .build()
        }
    }

    #[cfg(feature = "serde")]
    impl ::core::convert::From<AuthenticationTokenPayload> for __AuthenticationTokenPayload {
        fn from(value: AuthenticationTokenPayload) -> Self {
            Self::builder()
                .user_id(value.user_id)
                .user_role(value.user_role)
                .expiry_timestamp(value.expiry_timestamp)
                .build()
        }
    }

    #[cfg(feature = "serde")]
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(transparent)]
    struct __Uuid([u8; 16]);

    #[::bon::bon]
    impl __Uuid {
        #[builder]
        pub fn new(value: [u8; 16]) -> Self {
            Self(value)
        }
    }

    impl ::core::ops::Deref for __Uuid {
        type Target = [u8; 16];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl ::core::convert::From<::domain::Uuid> for __Uuid {
        fn from(value: ::domain::Uuid) -> Self {
            Self::builder().value(*value).build()
        }
    }

    impl ::core::convert::From<__Uuid> for ::domain::Uuid {
        fn from(value: __Uuid) -> Self {
            Self::builder().value(*value).build()
        }
    }
}
